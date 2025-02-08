use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::Uuid;

use crate::constants::{PREFIX_GET_WEBSITE_BY_ID, PREFIX_GET_WEBSITE_BY_SUBDOMAIN};
use crate::models::{async_redis_cache, User};
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn get_by_id(core_context: &CoreContext, id: Uuid, user: Option<&User>) -> sqlx::Result<Self> {
        let website = get_website_by_id(core_context, id).await?;

        if let Some(user) = user {
            if user.id != website.user_id {
                return Err(sqlx::Error::RowNotFound);
            }
        }

        Ok(website)
    }

    pub async fn get_by_id_with_search_rank(
        core_context: &CoreContext,
        id: Uuid,
        user: Option<&User>,
        query: &str,
    ) -> sqlx::Result<Self> {
        let user_id = user.map(|user| user.id);

        query_as!(
            Self,
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

    pub async fn get_by_subdomain(core_context: &CoreContext, subdomain: &str) -> sqlx::Result<Self> {
        get_website_by_subdomain(core_context, subdomain).await
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, Website>",
    create = r##" { async_redis_cache(PREFIX_GET_WEBSITE_BY_ID).await } "##
)]
pub(crate) async fn get_website_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Website> {
    query_as!(
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

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ subdomain.to_lowercase() }"#,
    ty = "AsyncRedisCache<String, Website>",
    create = r##" { async_redis_cache(PREFIX_GET_WEBSITE_BY_SUBDOMAIN).await } "##
)]
pub(crate) async fn get_website_by_subdomain(core_context: &CoreContext, subdomain: &str) -> sqlx::Result<Website> {
    if subdomain.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    query_as!(
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_uuid, insert_test_user, insert_test_website, setup_core_context};

    use super::Website;

    #[tokio::test]
    async fn should_get_website_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let result = Website::get_by_id(&core_context, website.id, Some(&user)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_when_user_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, None).await;

        let result = Website::get_by_id(&core_context, website.id, Some(&user)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = Website::get_by_id(&core_context, id, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_website_by_id_with_search_rank() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = Website::get_by_id_with_search_rank(&core_context, website.id, None, &website.name).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_website_by_id_with_search_rank_when_id_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = Website::get_by_id_with_search_rank(&core_context, id, None, "").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_not_get_website_by_subdomain_when_is_not_published() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = Website::get_by_subdomain(&core_context, &website.subdomain).await;

        assert!(result.is_err());
    }
}
