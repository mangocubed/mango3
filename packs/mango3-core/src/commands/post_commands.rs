use uuid::Uuid;

use crate::models::*;
use crate::utils::*;
use crate::CoreContext;

#[cfg(feature = "insert-post")]
use crate::enums::{Input, InputError};

#[cfg(feature = "insert-post")]
impl Validator {
    fn validate_post_title(&mut self, value: &str) -> bool {
        self.validate_presence(Input::Title, value)
            && self.validate_length(Input::Title, value, None, Some(256))
            && self.custom_validation(Input::Title, InputError::IsInvalid, &|| Uuid::try_parse(value).is_err())
    }

    async fn validate_post_slug(
        &mut self,
        core_context: &CoreContext,
        post: Option<&Post>,
        website: &Website,
        slug: &str,
    ) -> bool {
        if self.validate_presence(Input::Slug, slug)
            && self.validate_format(Input::Slug, slug, &crate::constants::REGEX_SLUG)
            && self.validate_length(Input::Slug, slug, None, Some(256))
            && self.custom_validation(Input::Slug, InputError::IsInvalid, &|| Uuid::try_parse(slug).is_err())
            && self.custom_validation(Input::Slug, InputError::IsInvalid, &|| {
                !crate::constants::BLACKLISTED_SLUGS.contains(&slug)
            })
        {
            let id = post.map(|p| p.id);
            let slug_exists = sqlx::query!(
                "SELECT id FROM posts WHERE ($1::uuid IS NULL OR id != $1) AND LOWER(slug) = $2 AND website_id = $3 LIMIT 1",
                id,         // $1
                slug,       // $2
                website.id  // $3
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            self.custom_validation(Input::Slug, InputError::AlreadyInUse, &|| !slug_exists)
        } else {
            false
        }
    }

    fn validate_post_content(&mut self, value: &str) -> bool {
        self.validate_length(
            Input::Content,
            value,
            None,
            Some(crate::config::MISC_CONFIG.max_post_content_length),
        )
    }

    fn validate_post_variables(&mut self, value: Option<&serde_json::Value>) -> bool {
        self.custom_validation(Input::Variables, InputError::IsInvalid, &|| value.is_some())
    }
}

#[cfg(any(feature = "clear-post-cache", feature = "get-post-by-slug"))]
fn cache_key_get_post_by_slug(slug: &str, website: &Website) -> String {
    format!("{}:{}", slug.to_lowercase(), website.id)
}

#[cfg(feature = "clear-post-cache")]
async fn clear_post_cache(core_context: &CoreContext, post: &Post) {
    use crate::constants::*;

    futures::future::join4(
        crate::models::POST_CONTENT_HTML.cache_remove(PREFIX_POST_CONTENT_HTML, &post.id),
        crate::models::POST_CONTENT_PREVIEW_HTML.cache_remove(PREFIX_POST_CONTENT_PREVIEW_HTML, &post.id),
        GET_CACHED_POST_BY_ID.cache_remove(PREFIX_GET_POST_BY_ID, &post.id),
        async {
            let website = post.website(core_context).await.expect("Could not get website");

            GET_POST_BY_SLUG
                .cache_remove(
                    PREFIX_GET_POST_BY_SLUG,
                    &cache_key_get_post_by_slug(&post.slug, &website),
                )
                .await
        },
    )
    .await;
}

#[cfg(feature = "delete-post")]
pub async fn delete_post(core_context: &CoreContext, post: &Post) -> MutResult {
    sqlx::query!("DELETE FROM posts WHERE id = $1", post.id)
        .execute(&core_context.db_pool)
        .await?;

    clear_post_cache(core_context, post).await;

    crate::mut_success!()
}

#[cfg(feature = "get-post-by-id")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "cached::AsyncRedisCache<Uuid, Post>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_POST_BY_ID).await } "##
)]
async fn get_cached_post_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Post> {
    sqlx::query_as!(
        Post,
        r#"SELECT
            id,
            website_id,
            user_id,
            language::varchar AS "language!",
            title,
            slug,
            content,
            variables,
            hashtag_ids,
            cover_image_blob_id,
            blob_ids,
            published_at,
            modified_at,
            NULL::real AS search_rank,
            created_at,
            updated_at
        FROM posts WHERE id = $1 LIMIT 1"#,
        id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-post-by-id")]
pub async fn get_post_by_id(
    core_context: &CoreContext,
    id: Uuid,
    website: Option<&Website>,
    user: Option<&User>,
    is_published: Option<bool>,
) -> sqlx::Result<Post> {
    let post = get_cached_post_by_id(core_context, id).await?;

    if let Some(user) = user {
        if user.id != post.user_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    if let Some(website) = website {
        if website.id != post.website_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    if let Some(is_published) = is_published {
        if is_published != post.is_published(core_context).await {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    Ok(post)
}

#[cfg(feature = "get-post-by-id-with-search-rank")]
pub async fn get_post_by_id_with_search_rank(
    core_context: &CoreContext,
    id: Uuid,
    website: Option<&Website>,
    user: Option<&User>,
    is_published: Option<bool>,
    query: &str,
) -> sqlx::Result<Post> {
    let website_id = website.map(|website| website.id);
    let user_id = user.map(|user| user.id);

    sqlx::query_as!(
        Post,
        r#"SELECT
            id,
            website_id,
            user_id,
            language::varchar AS "language!",
            title,
            slug,
            content,
            variables,
            hashtag_ids,
            cover_image_blob_id,
            blob_ids,
            published_at,
            modified_at,
            ts_rank(search, websearch_to_tsquery($5)) AS search_rank,
            created_at,
            updated_at
        FROM posts
        WHERE id = $1 AND ($2::uuid IS NULL OR website_id = $2)
            AND ($3::uuid IS NULL OR user_id = $3)
            AND (
                $4::bool IS NULL OR ($4 IS TRUE AND published_at IS NOT NULL)
                OR ($4 IS FALSE AND published_at IS NULL)
            )
        LIMIT 1"#,
        id,           // $1
        website_id,   // $2
        user_id,      // $3
        is_published, // $4
        query,        // $5
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-post-by-slug")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ cache_key_get_post_by_slug(slug, website) }"#,
    ty = "cached::AsyncRedisCache<String, Post>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_POST_BY_SLUG).await } "##
)]
pub async fn get_post_by_slug(core_context: &CoreContext, slug: &str, website: &Website) -> sqlx::Result<Post> {
    if slug.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        Post,
        r#"SELECT
            id,
            website_id,
            user_id,
            language::varchar as "language!",
            title,
            slug,
            content,
            variables,
            hashtag_ids,
            cover_image_blob_id,
            blob_ids,
            published_at,
            modified_at,
            NULL::real AS search_rank,
            created_at,
            updated_at
        FROM posts WHERE slug = $1 AND website_id = $2 AND published_at IS NOT NULL LIMIT 1"#,
        slug,       // $1
        website.id  // $2
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "insert-post")]
pub async fn insert_post(
    core_context: &CoreContext,
    website: &Website,
    user: &User,
    title: &str,
    slug: &str,
    content: &str,
    variables: &str,
    blobs: Vec<Blob<'_>>,
    cover_image_blob: Option<&Blob<'_>>,
    publish: bool,
) -> MutResult<Post> {
    let mut validator = crate::validator!();

    let title = title.trim();
    let slug = slug.trim().to_lowercase();
    let content = content.trim();
    let variables = variables.parse::<serde_json::Value>().ok();
    let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

    let hashtags = super::get_or_insert_many_hashtags(content).await?;
    let hashtag_ids = hashtags.data.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();
    let blob_ids = blobs.iter().map(|blob| blob.id).collect::<Vec<Uuid>>();

    validator.validate_post_title(title);
    validator.validate_post_slug(core_context, None, website, &slug).await;
    validator.validate_post_content(content);
    validator.validate_post_variables(variables.as_ref());

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let result = sqlx::query_as!(
        Post,
        r#"INSERT INTO posts (
                website_id,
                user_id,
                title,
                slug,
                content,
                variables,
                hashtag_ids,
                cover_image_blob_id,
                blob_ids,
                published_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CASE WHEN $10 IS TRUE THEN current_timestamp ELSE NULL END)
            RETURNING
                id,
                website_id,
                user_id,
                language::varchar as "language!",
                title,
                slug,
                content,
                variables,
                hashtag_ids,
                cover_image_blob_id,
                blob_ids,
                published_at,
                modified_at,
                NULL::real AS search_rank,
                created_at,
                updated_at"#,
        website.id,          // $1
        user.id,             // $2
        title,               // $3
        slug,                // $4
        content,             // $5
        variables.unwrap(),  // $6
        &hashtag_ids,        // $7
        cover_image_blob_id, // $8
        &blob_ids,           // $9
        publish,             // $10
    )
    .fetch_one(&core_context.db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(feature = "paginate-posts")]
pub async fn paginate_posts<'a>(
    core_context: &'a CoreContext,
    page_params: &CursorPageParams,
    website: Option<&'a Website>,
    user: Option<&'a User>,
    hashtag: Option<&'a Hashtag<'_>>,
    is_published: Option<bool>,
) -> CursorPage<Post> {
    crate::cursor_page!(
        core_context,
        page_params,
        |node: Post| node.id,
        move |core_context, after| async move {
            get_post_by_id(core_context, after, website, user, is_published)
                .await
                .ok()
        },
        move |core_context, cursor_resource, limit| async move {
            let website_id = website.map(|w| w.id);
            let user_id = user.map(|u| u.id);
            let hashtag_id = hashtag.map(|h| h.id);
            let (cursor_id, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), Some(c.created_at)))
                .unwrap_or_default();

            sqlx::query_as!(
                Post,
                r#"SELECT
                        id,
                        website_id,
                        user_id,
                        language::varchar as "language!",
                        title,
                        slug,
                        content,
                        variables,
                        hashtag_ids,
                        cover_image_blob_id,
                        blob_ids,
                        published_at,
                        modified_at,
                        NULL::real AS search_rank,
                        created_at,
                        updated_at
                    FROM posts
                    WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                        AND ($3::uuid IS NULL OR $3 = ANY(hashtag_ids)) AND (
                            $4::bool IS NULL OR ($4 IS TRUE AND published_at IS NOT NULL)
                            OR ($4 IS FALSE AND published_at IS NULL)
                        ) AND ($6::timestamptz IS NULL OR created_at < $6 OR (created_at = $6 AND id < $5))
                    ORDER BY created_at DESC, id DESC LIMIT $7"#,
                website_id,        // $1
                user_id,           // $2
                hashtag_id,        // $3
                is_published,      // $4
                cursor_id,         // $5
                cursor_created_at, // $6
                limit,             // $7
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "search-posts")]
pub async fn search_posts<'a>(
    core_context: &'a CoreContext,
    cursor_page_params: &CursorPageParams,
    website: Option<&'a Website>,
    user: Option<&'a User>,
    is_published: Option<bool>,
    query: &'a str,
) -> CursorPage<Post> {
    crate::cursor_page!(
        core_context,
        cursor_page_params,
        |node: Post| node.id,
        move |core_context, after| async move {
            get_post_by_id_with_search_rank(core_context, after, website, user, is_published, query)
                .await
                .ok()
        },
        move |core_context, cursor_resource, limit| async move {
            let website_id = website.map(|w| w.id);
            let user_id = user.map(|u| u.id);
            let (cursor_id, cursor_search_rank, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), c.search_rank, Some(c.created_at)))
                .unwrap_or_default();

            sqlx::query_as!(
                Post,
                r#"SELECT
                    id,
                    website_id,
                    user_id,
                    language::varchar as "language!",
                    title,
                    slug,
                    content,
                    variables,
                    hashtag_ids,
                    cover_image_blob_id,
                    blob_ids,
                    published_at,
                    modified_at,
                    ts_rank(search, websearch_to_tsquery($4)) AS search_rank,
                    created_at,
                    updated_at
                FROM posts
                WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                    AND (
                        $3::bool IS NULL OR ($3 IS TRUE AND published_at IS NOT NULL)
                        OR ($3 IS FALSE AND published_at IS NULL)
                    ) AND (
                        search @@ websearch_to_tsquery($4)
                        OR title ILIKE '%' || $4 || '%'
                        OR slug ILIKE '%' || $4 || '%'
                        OR content ILIKE '%' || $4 || '%'
                    ) AND (
                        ($5::uuid IS NULL OR $6::real IS NULL OR $7::timestamptz IS NULL)
                        OR ts_rank(search, websearch_to_tsquery($4)) < $6 OR (
                            ts_rank(search, websearch_to_tsquery($4)) = $6 AND (
                                created_at < $7 OR (created_at = $7 AND id < $5)
                            )
                        )
                    )
                ORDER BY search_rank DESC, created_at DESC, id DESC LIMIT $8"#,
                website_id,         // $1
                user_id,            // $2
                is_published,       // $3
                query,              // $4
                cursor_id,          // $5
                cursor_search_rank, // $6
                cursor_created_at,  // $7
                limit,              // $8
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "update-post")]
pub async fn update_post(
    core_context: &CoreContext,
    post: &Post,
    title: &str,
    slug: &str,
    content: &str,
    variables: &str,
    blobs: Vec<Blob<'_>>,
    cover_image_blob: Option<&Blob<'_>>,
    publish: bool,
) -> crate::utils::MutResult<Post> {
    let mut validator = crate::validator!();

    let title = title.trim();
    let slug = slug.trim().to_lowercase();
    let content = content.trim();
    let variables = variables.parse::<serde_json::Value>().ok();
    let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);

    let hashtags = super::get_or_insert_many_hashtags(content).await?;
    let hashtag_ids = hashtags.data.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();
    let blob_ids = blobs.iter().map(|blob| blob.id).collect::<Vec<Uuid>>();

    validator.validate_post_title(title);
    validator
        .validate_post_slug(
            core_context,
            Some(post),
            &post.website(core_context).await.unwrap(),
            &slug,
        )
        .await;
    validator.validate_post_content(content);
    validator.validate_post_variables(variables.as_ref());

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let result = sqlx::query_as!(
        Post,
        r#"UPDATE posts SET
            title = $2,
            slug = $3,
            content = $4,
            variables = $5,
            hashtag_ids = $6,
            cover_image_blob_id = $7,
            blob_ids = $8,
            published_at = CASE
                WHEN $9 IS TRUE AND published_at IS NOT NULL THEN published_at
                WHEN $9 IS TRUE THEN current_timestamp
                ELSE NULL
            END,
            modified_at = CASE WHEN $9 IS TRUE THEN current_timestamp ELSE NULL END
        WHERE id = $1
        RETURNING
            id,
            website_id,
            user_id,
            language::varchar as "language!",
            title,
            slug,
            content,
            variables,
            hashtag_ids,
            cover_image_blob_id,
            blob_ids,
            published_at,
            modified_at,
            NULL::real AS search_rank,
            created_at,
            updated_at"#,
        post.id,             // $1
        title,               // $2
        slug,                // $3
        content,             // $4
        variables,           // $5
        &hashtag_ids,        // $6
        cover_image_blob_id, // $7
        &blob_ids,           // $8
        publish,             // $9
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(post1) => {
            clear_post_cache(core_context, post).await;

            crate::mut_success!(post1)
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_post, insert_test_user, insert_test_website, setup_core_context};
    use crate::utils::CursorPageParams;

    use super::{
        delete_post, get_post_by_id, get_post_by_id_with_search_rank, get_post_by_slug, paginate_posts, search_posts,
    };

    #[tokio::test]
    async fn should_delete_post() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = delete_post(&core_context, &post).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_post_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let result = get_post_by_id(&core_context, post.id, Some(&website), Some(&user), None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_when_website_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, None, Some(&user)).await;

        let result = get_post_by_id(&core_context, post.id, Some(&website), Some(&user), None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_post_by_id(&core_context, id, None, None, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_post_by_id_with_search_rank() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = get_post_by_id_with_search_rank(&core_context, post.id, None, None, None, &post.title).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_with_search_rank_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_post_by_id_with_search_rank(&core_context, id, None, None, None, "").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_post_by_slug_when_is_not_published() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = get_post_by_slug(&core_context, &post.slug, &website).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = paginate_posts(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            None,
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page = paginate_posts(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            None,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }

    #[tokio::test]
    async fn should_find_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = search_posts(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            "",
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_find_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page = search_posts(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            &post.title,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
