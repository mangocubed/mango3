use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobPresenter, HashtagPresenter};

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsitePresenter {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub description_preview_html: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub initials: String,
    pub icon_image_blob: Option<BlobPresenter>,
    pub text_icon_url: Url,
    pub cover_image_blob: Option<BlobPresenter>,
    pub light_theme: String,
    pub dark_theme: String,
    pub is_published: bool,
    pub host: String,
    pub url: Url,

    #[cfg(feature = "website-description-html")]
    pub description_html: String,
    #[cfg(feature = "website-storage")]
    pub available_storage_str: String,
    #[cfg(feature = "website-storage")]
    pub max_storage_str: String,
    #[cfg(feature = "website-storage")]
    pub used_storage_str: String,
    #[cfg(feature = "website-storage")]
    pub available_storage: i64,
    #[cfg(feature = "website-storage")]
    pub max_storage: i64,
    #[cfg(feature = "website-storage")]
    pub used_storage: i64,
}

impl WebsitePresenter {
    pub fn icon_image_url(&self, size: u16) -> Url {
        self.icon_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| {
                let mut text_icon_url = self.text_icon_url.clone();
                text_icon_url.set_query(Some(&format!("size={size}")));
                text_icon_url
            })
    }
}

#[cfg(feature = "ssr")]
impl FromModel<Website> for WebsitePresenter {
    fn from_model(core_context: &CoreContext, website: &Website) -> impl std::future::Future<Output = Self> {
        async move {
            let hashtags = futures::future::join_all(
                website
                    .hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };
            let cover_image_blob = if let Some(Ok(blob)) = website.cover_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };

            #[cfg(feature = "website-storage")]
            let (available_storage, max_storage, used_storage) = (
                website.available_storage(core_context).await,
                website.max_storage(),
                website.used_storage(core_context).await,
            );

            Self {
                id: website.id,
                name: website.name.clone(),
                description: website.description.clone(),
                description_preview_html: website.description_preview_html().await,
                hashtags,
                initials: website.initials(),
                icon_image_blob,
                text_icon_url: website.text_icon_url(),
                cover_image_blob,
                light_theme: website.light_theme.clone(),
                dark_theme: website.dark_theme.clone(),
                is_published: website.is_published(),
                host: website.host(),
                url: website.url(),

                #[cfg(feature = "website-description-html")]
                description_html: website.description_html().await,
                #[cfg(feature = "website-storage")]
                available_storage_str: available_storage.to_string(),
                #[cfg(feature = "website-storage")]
                max_storage_str: max_storage.to_string(),
                #[cfg(feature = "website-storage")]
                used_storage_str: used_storage.to_string(),
                #[cfg(feature = "website-storage")]
                available_storage: available_storage.bytes(),
                #[cfg(feature = "website-storage")]
                max_storage: max_storage.bytes(),
                #[cfg(feature = "website-storage")]
                used_storage: used_storage.bytes(),
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WebsiteMinPresenter {
    pub id: Uuid,
    pub name: String,
    pub description_preview_html: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub initials: String,
    pub icon_image_blob: Option<BlobPresenter>,
    pub text_icon_url: Url,
    pub is_published: bool,
    pub host: String,
    pub url: Url,
}

impl WebsiteMinPresenter {
    pub fn icon_image_url(&self, size: u16) -> Url {
        self.icon_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| {
                let mut text_icon_url = self.text_icon_url.clone();
                text_icon_url.set_query(Some(&format!("size={size}")));
                text_icon_url
            })
    }
}

#[cfg(feature = "ssr")]
impl FromModel<Website> for WebsiteMinPresenter {
    fn from_model(core_context: &CoreContext, website: &Website) -> impl std::future::Future<Output = Self> {
        async move {
            let hashtags = futures::future::join_all(
                website
                    .hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };

            Self {
                id: website.id,
                name: website.name.clone(),
                description_preview_html: website.description_preview_html().await,
                hashtags,
                initials: website.initials(),
                icon_image_blob,
                text_icon_url: website.text_icon_url(),
                is_published: website.is_published(),
                host: website.host(),
                url: website.url(),
            }
        }
    }
}

impl From<&WebsitePresenter> for WebsiteMinPresenter {
    fn from(website: &WebsitePresenter) -> Self {
        WebsiteMinPresenter {
            id: website.id,
            description_preview_html: website.description_preview_html.clone(),
            name: website.name.clone(),
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

impl From<WebsitePresenter> for WebsiteMinPresenter {
    fn from(website: WebsitePresenter) -> Self {
        Self::from(&website)
    }
}
