use std::fmt::Display;

use serde::{Deserialize, Serialize};
use sqlx::query;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::BASIC_CONFIG;
use crate::CoreContext;

use super::{Blob, Hashtag, User};

#[derive(Clone, Deserialize, Serialize)]
pub struct Website {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub subdomain: String,
    pub description: String,
    pub hashtag_ids: Vec<Uuid>,
    pub icon_image_blob_id: Option<Uuid>,
    pub cover_image_blob_id: Option<Uuid>,
    pub light_theme: String,
    pub dark_theme: String,
    pub language: String,
    pub published_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Website {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Website {
    pub async fn count(core_context: &CoreContext, user: Option<&User>) -> sqlx::Result<i64> {
        let user_id = user.map(|u| u.id);

        query!(
            "SELECT COUNT(*) FROM websites WHERE $1::uuid IS NULL OR user_id = $1 LIMIT 1",
            user_id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
    }

    pub async fn cover_image_blob(&self) -> Option<sqlx::Result<Blob<'_>>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(crate::commands::get_blob_by_id(id, None, None).await)
        } else {
            None
        }
    }

    #[cfg(feature = "website-description-html")]
    pub async fn description_html(&self) -> String {
        website_description_html(self).await.unwrap_or_default()
    }

    pub async fn description_preview_html(&self) -> String {
        website_description_preview_html(self).await.unwrap_or_default()
    }

    pub async fn hashtags(&self) -> Vec<Hashtag> {
        crate::commands::all_hashtags_by_ids(&self.hashtag_ids).await
    }

    pub fn host(&self) -> String {
        self.url().host().unwrap().to_string()
    }

    pub async fn icon_image_blob(&self) -> Option<sqlx::Result<Blob<'_>>> {
        if let Some(id) = self.icon_image_blob_id {
            Some(crate::commands::get_blob_by_id(id, None, None).await)
        } else {
            None
        }
    }

    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    pub fn text_icon_url(&self) -> Url {
        BASIC_CONFIG.text_icon_url(&self.initials())
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.website_url(&self.subdomain)
    }

    #[cfg(feature = "website-storage")]
    pub async fn available_storage(&self, core_context: &CoreContext) -> size::Size {
        self.max_storage() - self.used_storage(core_context).await
    }

    #[cfg(feature = "website-storage")]
    pub fn max_storage(&self) -> size::Size {
        crate::config::WEBSITE_CONFIG.max_storage
    }

    #[cfg(feature = "website-storage")]
    pub async fn used_storage(&self, core_context: &CoreContext) -> size::Size {
        crate::commands::get_used_website_storage(core_context, self)
            .await
            .expect("Could not get used storage")
    }
}

#[cfg(feature = "website-description-html")]
#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ website.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_WEBSITE_DESCRIPTION_HTML).await } "##
)]
pub(crate) async fn website_description_html(website: &Website) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(&website.description, true))
}

#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ website.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_WEBSITE_DESCRIPTION_PREVIEW_HTML).await } "##
)]
pub(crate) async fn website_description_preview_html(website: &Website) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(
        &website
            .description
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false
    ))
}
