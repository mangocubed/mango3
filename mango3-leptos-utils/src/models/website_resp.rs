use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobResp, HashtagResp};

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsiteResp {
    pub id: String,
    pub name: String,
    pub description: String,

    #[cfg(feature = "website_description_html")]
    pub description_html: String,

    #[cfg(feature = "website_description_preview_html")]
    pub description_preview_html: String,

    pub hashtags: Vec<HashtagResp>,
    pub initials: String,
    pub icon_image_blob: Option<BlobResp>,
    pub text_icon_url: String,
    pub cover_image_blob: Option<BlobResp>,
    pub light_theme: String,
    pub dark_theme: String,
    pub is_published: bool,
    pub host: String,
    pub url: String,

    #[cfg(feature = "website_storage")]
    pub available_storage_str: String,

    #[cfg(feature = "website_storage")]
    pub max_storage_str: String,

    #[cfg(feature = "website_storage")]
    pub used_storage_str: String,

    #[cfg(feature = "website_storage")]
    pub available_storage: i64,

    #[cfg(feature = "website_storage")]
    pub max_storage: i64,

    #[cfg(feature = "website_storage")]
    pub used_storage: i64,
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
        let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };
        let cover_image_blob = if let Some(Ok(blob)) = website.cover_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };

        #[cfg(feature = "website_storage")]
        let (available_storage, max_storage, used_storage) = (
            website.available_storage(core_context).await,
            website.max_storage(),
            website.used_storage(core_context).await,
        );

        Self {
            id: website.id.to_string(),
            name: website.name.clone(),
            description: website.description.clone(),

            #[cfg(feature = "website_description_html")]
            description_html: website.description_html().await,

            #[cfg(feature = "website_description_preview_html")]
            description_preview_html: website.description_preview_html().await,
            hashtags: website
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            initials: website.initials(),
            icon_image_blob,
            text_icon_url: website.text_icon_url().to_string(),
            cover_image_blob,
            light_theme: website.light_theme.clone(),
            dark_theme: website.dark_theme.clone(),
            is_published: website.is_published(),
            host: website.host(),
            url: website.url().to_string(),

            #[cfg(feature = "website_storage")]
            available_storage_str: available_storage.to_string(),

            #[cfg(feature = "website_storage")]
            max_storage_str: max_storage.to_string(),

            #[cfg(feature = "website_storage")]
            used_storage_str: used_storage.to_string(),

            #[cfg(feature = "website_storage")]
            available_storage: available_storage.bytes(),

            #[cfg(feature = "website_storage")]
            max_storage: max_storage.bytes(),

            #[cfg(feature = "website_storage")]
            used_storage: used_storage.bytes(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsitePreviewResp {
    pub id: String,
    pub name: String,

    #[cfg(feature = "website_description_preview_html")]
    pub description_preview_html: String,

    pub hashtags: Vec<HashtagResp>,
    pub initials: String,
    pub icon_image_blob: Option<BlobResp>,
    pub text_icon_url: String,
    pub is_published: bool,
    pub host: String,
    pub url: String,
}

impl WebsitePreviewResp {
    pub fn icon_image_url(&self, size: u16) -> String {
        self.icon_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| format!("{}?size={}", self.text_icon_url, size))
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Website> for WebsitePreviewResp {
    async fn from_core(core_context: &CoreContext, website: &Website) -> Self {
        let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };

        Self {
            id: website.id.to_string(),
            name: website.name.clone(),

            #[cfg(feature = "website_description_preview_html")]
            description_preview_html: website.description_preview_html().await,

            hashtags: website
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            initials: website.initials(),
            icon_image_blob,
            text_icon_url: website.text_icon_url().to_string(),
            is_published: website.is_published(),
            host: website.host(),
            url: website.url().to_string(),
        }
    }
}

impl From<&WebsiteResp> for WebsitePreviewResp {
    fn from(website: &WebsiteResp) -> Self {
        Self {
            id: website.id.clone(),
            name: website.name.clone(),

            #[cfg(feature = "website_description_preview_html")]
            description_preview_html: website.description_preview_html.clone(),

            hashtags: website.hashtags.clone(),
            initials: website.initials.clone(),
            icon_image_blob: website.icon_image_blob.clone(),
            text_icon_url: website.text_icon_url.clone(),
            is_published: website.is_published,
            host: website.host.clone(),
            url: website.url.clone(),
        }
    }
}

impl From<WebsiteResp> for WebsitePreviewResp {
    fn from(website: WebsiteResp) -> Self {
        Self::from(&website)
    }
}

impl From<Arc<WebsiteResp>> for WebsitePreviewResp {
    fn from(website: Arc<WebsiteResp>) -> Self {
        Self::from(&Arc::into_inner(website).expect("Could not unwrap website"))
    }
}
