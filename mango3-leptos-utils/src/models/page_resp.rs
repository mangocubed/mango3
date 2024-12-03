use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Page;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::{parse_html, FromCore};

#[derive(Clone, Deserialize, Serialize)]
pub struct PageResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content_html: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Page> for PageResp {
    async fn from_core(core_context: &CoreContext, page: &Page) -> Self {
        Self {
            id: page.id.to_string(),
            title: page.title.clone(),
            slug: page.slug.clone(),
            content_html: parse_html(&page.content),
            cover_image_blob: page
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: page.is_published(core_context).await,
            url: page.url(&core_context).await.to_string(),
            published_at: page.published_at,
            created_at: page.created_at,
            updated_at: page.updated_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PagePreviewResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content_preview_html: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Page> for PagePreviewResp {
    async fn from_core(core_context: &CoreContext, page: &Page) -> Self {
        Self {
            id: page.id.to_string(),
            title: page.title.clone(),
            slug: page.slug.clone(),
            content_preview_html: parse_html(&page.content_preview()),
            cover_image_blob: page
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: page.is_published(core_context).await,
            url: page.url(&core_context).await.to_string(),
            created_at: page.created_at,
            updated_at: page.updated_at,
        }
    }
}
