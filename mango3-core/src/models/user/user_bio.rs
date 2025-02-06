use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::async_redis_cache;
use crate::utils::parse_html;

use super::User;

impl User {
    pub async fn bio_html(&self) -> String {
        user_bio_html(self).await.unwrap_or_default()
    }
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ user.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache().await } "##
)]
pub(crate) async fn user_bio_html(user: &User) -> Result<String, RedisCacheError> {
    Ok(parse_html(&user.bio, true))
}
