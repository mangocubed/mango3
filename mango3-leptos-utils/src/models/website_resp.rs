use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::{parse_html, FromCore};

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsiteResp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub description_html: String,
    pub description_preview_html: String,
    pub initials: String,
    pub icon_image_blob: Option<BlobResp>,
    pub text_icon_url: String,
    pub cover_image_blob: Option<BlobResp>,
    pub light_theme: String,
    pub dark_theme: String,
    pub is_published: bool,
    pub host: String,
    pub url: String,
}

impl WebsiteResp {
    pub fn icon_image_url(&self, size: u16) -> String {
        self.icon_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| format!("{}?size={}", self.text_icon_url, size))
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Website> for WebsiteResp {
    async fn from_core(core_context: &CoreContext, website: &Website) -> Self {
        Self {
            id: website.id.to_string(),
            name: website.name.clone(),
            description: website.description.clone(),
            description_html: parse_html(&website.description, true),
            description_preview_html: parse_html(&website.description_preview(), false),
            initials: website.initials(),
            icon_image_blob: website
                .icon_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            text_icon_url: website.text_icon_url().to_string(),
            cover_image_blob: website
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            light_theme: website.light_theme.clone(),
            dark_theme: website.dark_theme.clone(),
            is_published: website.is_published(),
            host: website.host(),
            url: website.url().to_string(),
        }
    }
}
