use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;

use crate::CoreContext;

use super::{Blob, Hashtag, User, Website};

#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub website_id: Uuid,
    pub user_id: Uuid,
    pub language: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub variables: Value,
    pub hashtag_ids: Vec<Uuid>,
    pub cover_image_blob_id: Option<Uuid>,
    pub blob_ids: Vec<Uuid>,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub search_rank: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Post {
    #[cfg(feature = "post-blobs")]
    pub async fn blobs(&self) -> Vec<Blob<'_>> {
        crate::commands::all_blobs_by_ids(self.blob_ids.clone(), None, None).await
    }

    pub async fn comments_count(&self, core_context: &CoreContext) -> i64 {
        crate::commands::get_post_comments_count(core_context, self).await
    }

    #[cfg(feature = "post-content-html")]
    pub async fn content_html(&self) -> String {
        post_content_html(self).await.unwrap_or_default()
    }

    pub async fn content_preview_html(&self) -> String {
        post_content_preview_html(self).await.unwrap_or_default()
    }

    pub async fn cover_image_blob(&self) -> Option<sqlx::Result<Blob<'_>>> {
        if let Some(id) = self.cover_image_blob_id {
            Some(crate::commands::get_blob_by_id(id, None, None).await)
        } else {
            None
        }
    }

    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        crate::commands::all_hashtags_by_ids(core_context, &self.hashtag_ids).await
    }

    pub async fn is_published(&self, core_context: &CoreContext) -> bool {
        self.website(core_context).await.unwrap().is_published() && self.published_at.is_some()
    }

    pub async fn reactions_count(&self, core_context: &CoreContext) -> i64 {
        crate::commands::get_post_reactions_count(core_context, self).await
    }

    pub async fn url(&self, core_context: &CoreContext) -> Url {
        self.website(core_context)
            .await
            .unwrap()
            .url()
            .join(&self.slug)
            .unwrap()
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }

    pub async fn views_count(&self, core_context: &CoreContext) -> i64 {
        crate::commands::get_post_views_count(core_context, self).await
    }

    pub async fn website(&self, core_context: &CoreContext) -> sqlx::Result<Website> {
        crate::commands::get_website_by_id(core_context, self.website_id, None).await
    }
}

#[cfg(feature = "post-content-html")]
#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_POST_CONTENT_HTML).await } "##
)]
pub(crate) async fn post_content_html(post: &Post) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(
        &crate::render_handlebars!(&post.content, &post.variables).unwrap_or_default(),
        true
    ))
}

#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ post.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_POST_CONTENT_PREVIEW_HTML).await } "##
)]
pub(crate) async fn post_content_preview_html(post: &Post) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(
        &crate::constants::REGEX_HANDLEBARS
            .replace_all(&post.content, "")
            .trim()
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false
    ))
}
