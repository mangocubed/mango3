use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::models::async_redis_cache;
use crate::utils::parse_html;

#[cfg(feature = "user_bio_html")]
use crate::constants::PREFIX_USER_BIO_HTML;
#[cfg(feature = "user_bio_preview_html")]
use crate::constants::PREFIX_USER_BIO_PREVIEW_HTML;

use super::User;

impl User {
    #[cfg(feature = "user_bio_html")]
    pub async fn bio_html(&self) -> String {
        user_bio_html(self).await.unwrap_or_default()
    }

    #[cfg(feature = "user_bio_preview_html")]
    pub async fn bio_preview_html(&self) -> String {
        user_bio_preview_html(self).await.unwrap_or_default()
    }
}

#[cfg(feature = "user_bio_html")]
#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ user.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_USER_BIO_HTML).await } "##
)]
pub(crate) async fn user_bio_html(user: &User) -> Result<String, RedisCacheError> {
    Ok(parse_html(&user.bio, true))
}

#[cfg(feature = "user_bio_preview_html")]
#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ user.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_USER_BIO_PREVIEW_HTML).await } "##
)]
pub(crate) async fn user_bio_preview_html(user: &User) -> Result<String, RedisCacheError> {
    Ok(parse_html(
        &user
            .bio
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false,
    ))
}
