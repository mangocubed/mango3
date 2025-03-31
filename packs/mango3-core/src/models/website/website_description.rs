use cached::proc_macro::io_cached;
use cached::stores::AsyncRedisCache;
use cached::RedisCacheError;
use sqlx::types::uuid::Uuid;

use crate::models::async_redis_cache;
use crate::utils::parse_html;

use crate::constants::PREFIX_WEBSITE_DESCRIPTION_HTML;
use crate::constants::PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML;

use super::Website;

impl Website {
    pub async fn description_html(&self) -> String {
        website_description_html(self).await.unwrap_or_default()
    }

    pub async fn description_preview_html(&self) -> String {
        website_description_preview_html(self).await.unwrap_or_default()
    }
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ website.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_WEBSITE_DESCRIPTION_HTML).await } "##
)]
pub(crate) async fn website_description_html(website: &Website) -> Result<String, RedisCacheError> {
    Ok(parse_html(&website.description, true))
}

#[io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ website.id }"#,
    ty = "AsyncRedisCache<Uuid, String>",
    create = r##" { async_redis_cache(PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML).await } "##
)]
pub(crate) async fn website_description_preview_html(website: &Website) -> Result<String, RedisCacheError> {
    Ok(parse_html(
        &website
            .description
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false,
    ))
}
