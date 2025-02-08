use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::uuid::Uuid;

use crate::constants::{PREFIX_GET_POST_BY_ID, PREFIX_GET_POST_BY_SLUG};
use crate::models::{async_redis_cache, User, Website};
use crate::CoreContext;

use super::Post;

impl Post {
    pub fn cache_key_get_by_slug(slug: &str, website: &Website) -> String {
        format!("{}:{}", slug.to_lowercase(), website.id)
    }

    pub async fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
        user: Option<&User>,
        is_published: Option<bool>,
    ) -> sqlx::Result<Self> {
        let post = get_post_by_id(core_context, id).await?;

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

    pub async fn get_by_id_with_search_rank(
        core_context: &CoreContext,
        id: Uuid,
        website: Option<&Website>,
        user: Option<&User>,
        is_published: Option<bool>,
        query: &str,
    ) -> sqlx::Result<Post> {
        let website_id = website.map(|website| website.id);
        let user_id = user.map(|user| user.id);
        query_as!(
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

    pub async fn get_by_slug(core_context: &CoreContext, slug: &str, website: &Website) -> sqlx::Result<Self> {
        get_post_by_slug(core_context, slug, website).await
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, Post>",
    create = r##" { async_redis_cache(PREFIX_GET_POST_BY_ID).await } "##
)]
pub(crate) async fn get_post_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Post> {
    query_as!(
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

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ Post::cache_key_get_by_slug(slug, website) }"#,
    ty = "AsyncRedisCache<String, Post>",
    create = r##" { async_redis_cache(PREFIX_GET_POST_BY_SLUG).await } "##
)]
pub(crate) async fn get_post_by_slug(core_context: &CoreContext, slug: &str, website: &Website) -> sqlx::Result<Post> {
    if slug.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    query_as!(
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    use super::Post;

    #[tokio::test]
    async fn should_get_post_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let result = Post::get_by_id(&core_context, post.id, Some(&website), Some(&user), None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_when_website_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, None, Some(&user)).await;

        let result = Post::get_by_id(&core_context, post.id, Some(&website), Some(&user), None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = Post::get_by_id(&core_context, id, None, None, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_post_by_id_with_search_rank() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = Post::get_by_id_with_search_rank(&core_context, post.id, None, None, None, &post.title).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_post_by_id_with_search_rank_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = Post::get_by_id_with_search_rank(&core_context, id, None, None, None, "").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_post_by_slug_when_is_not_published() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = Post::get_by_slug(&core_context, &post.slug, &website).await;

        assert!(result.is_err());
    }
}
