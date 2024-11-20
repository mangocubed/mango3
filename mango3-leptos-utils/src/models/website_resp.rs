use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsiteResp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub initials: String,
    pub icon_image_blob: Option<BlobResp>,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Website> for WebsiteResp {
    async fn from_core(core_context: &CoreContext, website: &Website) -> Self {
        Self {
            id: website.id.to_string(),
            name: website.name.clone(),
            description: website.description.clone(),
            initials: website.initials(),
            icon_image_blob: website
                .icon_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            cover_image_blob: website
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: website.is_published(),
            url: website.url().to_string(),
        }
    }
}