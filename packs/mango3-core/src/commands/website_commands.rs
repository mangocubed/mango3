use uuid::Uuid;

use crate::models::*;
use crate::CoreContext;

#[cfg(feature = "insert-website")]
use crate::enums::{Input, InputError};
#[cfg(feature = "insert-website")]
use crate::utils::{Validator, ValidatorTrait};

#[cfg(feature = "insert-website")]
impl Validator {
    async fn validate_website_name(
        &mut self,
        core_context: &CoreContext,
        website: Option<&Website>,
        value: &str,
    ) -> bool {
        if self.validate_presence(Input::Name, value)
            && self.validate_length(Input::Name, value, Some(3), Some(256))
            && self.custom_validation(Input::Name, InputError::IsInvalid, &|| Uuid::try_parse(value).is_err())
        {
            let id = website.map(|w| w.id);
            let name_exists = sqlx::query!(
                "SELECT id FROM websites WHERE ($1::uuid IS NULL OR id != $1) AND LOWER(name) = $2 LIMIT 1",
                id,                   // $1
                value.to_lowercase()  // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            self.custom_validation(Input::Name, InputError::AlreadyInUse, &|| !name_exists)
        } else {
            false
        }
    }

    fn validate_website_description(&mut self, value: &str) -> bool {
        self.validate_length(Input::Description, value, None, Some(1024))
    }
}

#[cfg(feature = "clear-website-cache")]
pub async fn clear_website_cache(website: &Website) {
    use crate::constants::*;
    use crate::utils::AsyncRedisCacheTrait;

    futures::future::join4(
        crate::models::WEBSITE_DESCRIPTION_HTML.cache_remove(PREFIX_WEBSITE_DESCRIPTION_HTML, &website.id),
        crate::models::WEBSITE_DESCRIPTION_PREVIEW_HTML
            .cache_remove(PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML, &website.id),
        GET_CACHED_WEBSITE_BY_ID.cache_remove(PREFIX_GET_WEBSITE_BY_ID, &website.id),
        GET_CACHED_WEBSITE_BY_SUBDOMAIN
            .cache_remove(PREFIX_GET_WEBSITE_BY_SUBDOMAIN, &website.subdomain.to_lowercase()),
    )
    .await;
}

#[cfg(feature = "delete-website")]
pub async fn delete_website(core_context: &CoreContext, website: &Website) -> crate::utils::MutResult {
    sqlx::query!("DELETE FROM websites WHERE id = $1", website.id)
        .execute(&core_context.db_pool)
        .await?;

    clear_website_cache(website).await;

    crate::mut_success!()
}

#[cfg(feature = "get-website-by-id")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "cached::AsyncRedisCache<Uuid, Website>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_WEBSITE_BY_ID).await } "##
)]
async fn get_cached_website_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Website> {
    sqlx::query_as!(
        Website,
        r#"SELECT
            id,
            user_id,
            name,
            subdomain,
            description,
            hashtag_ids,
            icon_image_blob_id,
            cover_image_blob_id,
            light_theme,
            dark_theme,
            language::varchar AS "language!",
            published_at,
            NULL::real AS search_rank,
            created_at,
            updated_at
        FROM websites WHERE id = $1 LIMIT 1"#,
        id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-website-by-subdomain")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ subdomain.to_lowercase() }"#,
    ty = "cached::AsyncRedisCache<String, Website>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_WEBSITE_BY_SUBDOMAIN).await } "##
)]
async fn get_cached_website_by_subdomain(core_context: &CoreContext, subdomain: &str) -> sqlx::Result<Website> {
    if subdomain.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        Website,
        r#"SELECT
            id,
            user_id,
            name,
            subdomain,
            description,
            hashtag_ids,
            icon_image_blob_id,
            cover_image_blob_id,
            light_theme,
            dark_theme,
            language::varchar AS "language!",
            published_at,
            NULL::real AS search_rank,
            created_at,
            updated_at
        FROM websites WHERE subdomain = $1 AND published_at IS NOT NULL LIMIT 1"#,
        subdomain // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-website-by-id")]
pub async fn get_website_by_id(core_context: &CoreContext, id: Uuid, user: Option<&User>) -> sqlx::Result<Website> {
    let website = get_cached_website_by_id(core_context, id).await?;

    if let Some(user) = user {
        if user.id != website.user_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    Ok(website)
}

#[cfg(feature = "get-website-by-id-with-search-rank")]
pub async fn get_website_by_id_with_search_rank(
    core_context: &CoreContext,
    id: Uuid,
    user: Option<&User>,
    query: &str,
) -> sqlx::Result<Website> {
    let user_id = user.map(|user| user.id);

    sqlx::query_as!(
        Website,
        r#"SELECT
                id,
                user_id,
                name,
                subdomain,
                description,
                hashtag_ids,
                icon_image_blob_id,
                cover_image_blob_id,
                light_theme,
                dark_theme,
                language::varchar AS "language!",
                published_at,
                ts_rank(search, websearch_to_tsquery($3)) AS search_rank,
                created_at,
                updated_at
            FROM websites WHERE id = $1 AND ($2::uuid IS NULL OR user_id = $2) LIMIT 1"#,
        id,      // $1
        user_id, // $2
        query,   // $3
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-used-website-storage")]
pub async fn get_used_website_storage(core_context: &CoreContext, website: &Website) -> sqlx::Result<size::Size> {
    sqlx::query!(
        "SELECT SUM(byte_size)::bigint AS total_size FROM blobs WHERE website_id = $1 LIMIT 1",
        website.id
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map(|record| size::Size::from_bytes(record.total_size.unwrap_or_default()))
}

#[cfg(feature = "get-website-by-subdomain")]
pub async fn get_website_by_subdomain(core_context: &CoreContext, subdomain: &str) -> sqlx::Result<Website> {
    get_cached_website_by_subdomain(core_context, subdomain).await
}

#[cfg(feature = "insert-website")]
pub async fn insert_website(
    core_context: &CoreContext,
    user: &User,
    name: &str,
    subdomain: &str,
    description: &str,
) -> crate::utils::MutResult<Website> {
    let mut validator = crate::validator!();

    let name = name.trim();
    let subdomain = subdomain.trim().to_lowercase();
    let description = description.trim();

    validator.validate_website_name(core_context, None, name).await;

    if validator.validate_presence(Input::Subdomain, &subdomain)
        && validator.validate_format(Input::Subdomain, &subdomain, &crate::constants::REGEX_SUBDOMAIN)
        && validator.validate_length(Input::Subdomain, &subdomain, Some(3), Some(256))
        && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
            Uuid::try_parse(&subdomain).is_err()
        })
        && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
            !crate::constants::BLACKLISTED_SLUGS.contains(&subdomain.as_str())
        })
    {
        let subdomain_exists = sqlx::query!(
            "SELECT id FROM websites WHERE LOWER(subdomain) = $1 LIMIT 1",
            subdomain // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok();
        validator.custom_validation(Input::Subdomain, InputError::AlreadyInUse, &|| !subdomain_exists);
    }

    validator.validate_website_description(description);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let hashtags = super::get_or_insert_many_hashtags(core_context, description).await?;
    let hashtag_ids = hashtags.data.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

    let result = sqlx::query_as!(
        Website,
        r#"INSERT INTO websites (user_id, name, subdomain, description, hashtag_ids) VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id,
            user_id,
            name,
            subdomain,
            description,
            hashtag_ids,
            icon_image_blob_id,
            cover_image_blob_id,
            light_theme,
            dark_theme,
            language::varchar AS "language!",
            published_at,
            NULL::real AS search_rank,
            created_at,
            updated_at"#,
        user.id,      // $1
        name,         // $2
        subdomain,    // $3
        description,  // $4
        &hashtag_ids, // $5
    )
    .fetch_one(&core_context.db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(feature = "paginate-websites-sorted-by-name-asc")]
pub async fn paginate_websites_sorted_by_name_asc<'a>(
    core_context: &'a CoreContext,
    page_params: &crate::utils::CursorPageParams,
    user: Option<&'a User>,
    is_published: Option<bool>,
) -> crate::utils::CursorPage<Website> {
    crate::cursor_page!(
        core_context,
        page_params,
        |node: Website| node.id,
        move |core_context, after| async move { get_website_by_id(core_context, after, user).await.ok() },
        move |core_context, cursor_resource, limit| async move {
            let user_id = user.map(|u| u.id);
            let cursor_name = cursor_resource.map(|c| c.name.clone());

            sqlx::query_as!(
                Website,
                r#"SELECT
                    id,
                    user_id,
                    name,
                    subdomain,
                    description,
                    hashtag_ids,
                    icon_image_blob_id,
                    cover_image_blob_id,
                    light_theme,
                    dark_theme,
                    language::varchar AS "language!",
                    published_at,
                    NULL::real AS search_rank,
                    created_at,
                    updated_at
                FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                    AND (
                        $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                        OR ($2 IS FALSE AND published_at IS NULL)
                    ) AND ($3::text IS NULL OR name > $3)
                ORDER BY name ASC LIMIT $4"#,
                user_id,      // $1
                is_published, // $2
                cursor_name,  // $3
                limit,        // $4
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "paginate-websites")]
pub async fn paginate_websites<'a>(
    core_context: &'a CoreContext,
    page_params: &crate::utils::CursorPageParams,
    user: Option<&'a User>,
    is_published: Option<bool>,
) -> crate::utils::CursorPage<Website> {
    crate::cursor_page!(
        core_context,
        page_params,
        |node: Website| node.id,
        move |core_context, after| async move { get_website_by_id(core_context, after, user).await.ok() },
        move |core_context, cursor_resource, limit| async move {
            let user_id = user.map(|u| u.id);
            let (cursor_id, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), Some(c.created_at)))
                .unwrap_or_default();

            sqlx::query_as!(
                Website,
                r#"SELECT
                    id,
                    user_id,
                    name,
                    subdomain,
                    description,
                    hashtag_ids,
                    icon_image_blob_id,
                    cover_image_blob_id,
                    light_theme,
                    dark_theme,
                    language::varchar AS "language!",
                    published_at,
                    NULL::real AS search_rank,
                    created_at,
                    updated_at
                FROM websites WHERE ($1::uuid IS NULL OR user_id = $1)
                    AND (
                        $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                        OR ($2 IS FALSE AND published_at IS NULL)
                    ) AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
                ORDER BY created_at DESC, id DESC LIMIT $5"#,
                user_id,           // $1
                is_published,      // $2
                cursor_id,         // $3
                cursor_created_at, // $4
                limit,             // $5
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "search-websites")]
pub async fn search_websites<'a>(
    core_context: &'a CoreContext,
    cursor_page_params: &crate::utils::CursorPageParams,
    user: Option<&'a User>,
    is_published: Option<bool>,
    query: &'a str,
) -> crate::utils::CursorPage<Website> {
    crate::cursor_page!(
        core_context,
        cursor_page_params,
        |node: Website| node.id,
        move |core_context, after| async move {
            get_website_by_id_with_search_rank(core_context, after, user, query)
                .await
                .ok()
        },
        move |core_context, cursor_resource, limit| async move {
            let user_id = user.map(|u| u.id);
            let (cursor_id, cursor_search_rank, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), c.search_rank, Some(c.created_at)))
                .unwrap_or_default();

            sqlx::query_as!(
                Website,
                r#"SELECT
                    id,
                    user_id,
                    name,
                    subdomain,
                    description,
                    hashtag_ids,
                    icon_image_blob_id,
                    cover_image_blob_id,
                    light_theme,
                    dark_theme,
                    language::varchar as "language!",
                    published_at,
                    ts_rank(search, websearch_to_tsquery($3)) AS search_rank,
                    created_at,
                    updated_at
                FROM websites
                WHERE ($1::uuid IS NULL OR user_id = $1)
                    AND (
                        $2::bool IS NULL OR ($2 IS TRUE AND published_at IS NOT NULL)
                        OR ($2 IS FALSE AND published_at IS NULL)
                    ) AND (
                        search @@ websearch_to_tsquery($3)
                        OR name ILIKE '%' || $3 || '%'
                        OR subdomain ILIKE '%' || $3 || '%'
                        OR description ILIKE '%' || $3 || '%'
                    ) AND (
                        ($4::uuid IS NULL OR $5::real IS NULL OR $6::timestamptz IS NULL)
                        OR ts_rank(search, websearch_to_tsquery($3)) < $5 OR (
                            ts_rank(search, websearch_to_tsquery($3)) = $5 AND (
                                created_at < $6 OR (created_at = $6 AND id < $4)
                            )
                        )
                    )
                ORDER BY search_rank DESC, created_at DESC, id DESC LIMIT $7"#,
                user_id,            // $1
                is_published,       // $2
                query,              // $3
                cursor_id,          // $4
                cursor_search_rank, // $5
                cursor_created_at,  // $6
                limit,              // $7
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "update-website")]
pub async fn update_website(
    core_context: &CoreContext,
    website: &Website,
    name: &str,
    description: &str,
    icon_image_blob: Option<&Blob>,
    cover_image_blob: Option<&Blob>,
    light_theme: &str,
    dark_theme: &str,
    publish: bool,
) -> crate::utils::MutResult<Website> {
    let mut validator = crate::validator!();

    let name = name.trim();
    let description = description.trim();
    let icon_image_blob_id = icon_image_blob.map(|blob| blob.id);
    let cover_image_blob_id = cover_image_blob.map(|blob| blob.id);
    let light_theme = light_theme.trim();
    let dark_theme = dark_theme.trim();

    validator.validate_website_name(core_context, Some(website), name).await;
    validator.validate_website_description(description);
    validator.custom_validation(Input::LightTheme, InputError::IsInvalid, &|| {
        crate::constants::LIGHT_THEMES.contains(&light_theme)
    });
    validator.custom_validation(Input::DarkTheme, InputError::IsInvalid, &|| {
        crate::constants::DARK_THEMES.contains(&dark_theme)
    });

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let hashtags = super::get_or_insert_many_hashtags(core_context, description).await?;
    let hashtag_ids = hashtags.data.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

    let result = sqlx::query_as!(
        Website,
        r#"UPDATE websites SET
            name = $2,
            description = $3,
            hashtag_ids = $4,
            icon_image_blob_id = $5,
            cover_image_blob_id = $6,
            light_theme = $7,
            dark_theme = $8,
            published_at = CASE
                WHEN $9 IS TRUE AND published_at IS NOT NULL THEN published_at
                WHEN $9 IS TRUE THEN current_timestamp
                ELSE NULL
            END
        WHERE id = $1 RETURNING
            id,
            user_id,
            name,
            subdomain,
            description,
            hashtag_ids,
            icon_image_blob_id,
            cover_image_blob_id,
            light_theme,
            dark_theme,
            language::varchar AS "language!",
            published_at,
            NULL::real AS search_rank,
            created_at,
            updated_at"#,
        website.id,          // $1
        name,                // $2
        description,         // $3
        &hashtag_ids,        // $4
        icon_image_blob_id,  // $5
        cover_image_blob_id, // $6
        light_theme,         // $7
        dark_theme,          // $8
        publish,             // $9
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(website1) => {
            clear_website_cache(website).await;

            crate::mut_success!(website1)
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_user, insert_test_website, setup_core_context};

    use super::{
        get_website_by_id, get_website_by_id_with_search_rank, get_website_by_subdomain, paginate_websites,
        paginate_websites_sorted_by_name_asc, search_websites,
    };

    #[tokio::test]
    async fn should_delete_website() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = website.delete(&core_context).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_website_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let result = get_website_by_id(&core_context, website.id, Some(&user)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_when_user_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, None).await;

        let result = get_website_by_id(&core_context, website.id, Some(&user)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_website_by_id(&core_context, id, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_website_by_id_with_search_rank() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = get_website_by_id_with_search_rank(&core_context, website.id, None, &website.name).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_with_search_rank_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_website_by_id_with_search_rank(&core_context, id, None, "").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_website_by_subdomain_when_is_not_published() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = get_website_by_subdomain(&core_context, &website.subdomain).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_zero_websites_sorted_by_created_at_desc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page = paginate_websites(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website_sorted_by_created_at_desc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = paginate_websites(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }

    #[tokio::test]
    async fn should_get_zero_websites_sorted_by_name_asc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page =
            paginate_websites_sorted_by_name_asc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website_sorted_by_name_asc() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        insert_test_website(&core_context, Some(&user)).await;

        let cursor_page =
            paginate_websites_sorted_by_name_asc(&core_context, &CursorPageParams::default(), Some(&user), None).await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }

    #[tokio::test]
    async fn should_get_zero_websites() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let cursor_page = search_websites(&core_context, &CursorPageParams::default(), Some(&user), None, "").await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_website() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = search_websites(
            &core_context,
            &CursorPageParams::default(),
            Some(&user),
            None,
            &website.name,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }

    #[tokio::test]
    async fn should_update_website() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;
        let name = fake_name();
        let description = fake_sentence();

        let result = website
            .update(&core_context, &name, &description, None, None, "light", "dark", true)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_update_website_when_fields_are_empty() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = website.update(&core_context, "", "", None, None, "", "", true).await;

        assert!(result.is_err());
    }
}
