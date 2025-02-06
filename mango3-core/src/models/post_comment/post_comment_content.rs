use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::async_redis_cache;
use crate::utils::parse_html;

use super::PostComment;

impl PostComment {
    pub async fn content_html(&self) -> String {
        post_comment_content_html(self).await.unwrap_or_default()
    }
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ comment.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache().await } "##
)]
pub(crate) async fn post_comment_content_html(comment: &PostComment) -> Result<String, RedisCacheError> {
    Ok(parse_html(&comment.content, true))
}
