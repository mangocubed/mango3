use uuid::Uuid;

use crate::models::{User, Website};
use crate::CoreContext;

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
            let name_exists = query!(
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
    future::join4(
        WEBSITE_DESCRIPTION_HTML.cache_remove(PREFIX_WEBSITE_DESCRIPTION_HTML, &website.id),
        WEBSITE_DESCRIPTION_PREVIEW_HTML.cache_remove(PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML, &website.id),
        GET_WEBSITE_BY_ID.cache_remove(PREFIX_GET_WEBSITE_BY_ID, &website.id),
        GET_WEBSITE_BY_SUBDOMAIN.cache_remove(PREFIX_GET_WEBSITE_BY_SUBDOMAIN, &website.subdomain.to_lowercase()),
    )
    .await;
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
    ty = "crate::AsyncRedisCache<String, Website>",
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
) -> Result<Self, ValidationErrors> {
    let mut validator = validator!();

    let name = name.trim();
    let subdomain = subdomain.trim().to_lowercase();
    let description = description.trim();

    validator.validate_website_name(core_context, None, name).await;

    if validator.validate_presence(Input::Subdomain, &subdomain)
        && validator.validate_format(Input::Subdomain, &subdomain, &REGEX_SUBDOMAIN)
        && validator.validate_length(Input::Subdomain, &subdomain, Some(3), Some(256))
        && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
            Uuid::try_parse(&subdomain).is_err()
        })
        && validator.custom_validation(Input::Subdomain, InputError::IsInvalid, &|| {
            !BLACKLISTED_SLUGS.contains(&subdomain.as_str())
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
        return Err(validator.errors);
    }

    let hashtags = Hashtag::get_or_insert_all(core_context, description).await?;
    let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

    sqlx::query_as!(
        Self,
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
