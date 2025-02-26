use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::models::async_redis_cache;

use crate::constants::{PREFIX_POST_CONTENT_PREVIEW_HTML, REGEX_HANDLEBARS};
use crate::utils::parse_html;

#[cfg(feature = "post_content_html")]
use crate::constants::PREFIX_POST_CONTENT_HTML;
#[cfg(feature = "handlebars")]
use crate::utils::render_handlebars;

use super::Post;

impl Post {
    #[cfg(feature = "post_content_html")]
    pub async fn content_html(&self) -> String {
        post_content_html(self).await.unwrap_or_default()
    }

    #[cfg(feature = "post_content_preview_html")]
    pub async fn content_preview_html(&self) -> String {
        post_content_preview_html(self).await.unwrap_or_default()
    }
}

#[cfg(feature = "post_content_html")]
#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_POST_CONTENT_HTML).await } "##
)]
pub(crate) async fn post_content_html(post: &Post) -> Result<String, RedisCacheError> {
    Ok(parse_html(
        &render_handlebars(&post.content, &post.variables).unwrap_or_default(),
        true,
    ))
}

#[cfg(feature = "post_content_preview_html")]
#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_POST_CONTENT_PREVIEW_HTML).await } "##
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
