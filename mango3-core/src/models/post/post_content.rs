use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::async_redis_cache;
use crate::constants::REGEX_HANDLEBARS;
use crate::utils::{parse_html, render_handlebars};

use super::Post;

impl Post {
    pub async fn content_html(&self) -> String {
        post_content_html(self).await.unwrap_or_default()
    }

    pub async fn content_preview_html(&self) -> String {
        post_content_preview_html(self).await.unwrap_or_default()
    }
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache("post_content_html").await } "##
)]
pub(crate) async fn post_content_html(post: &Post) -> Result<String, RedisCacheError> {
    Ok(parse_html(
        &render_handlebars(&post.content, &post.variables).unwrap_or_default(),
        true,
    ))
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache("post_content_preview_html").await } "##
)]
pub(crate) async fn post_content_preview_html(post: &Post) -> Result<String, RedisCacheError> {
    Ok(parse_html(
        &REGEX_HANDLEBARS
            .replace_all(&post.content, "")
            .trim()
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false,
    ))
}
