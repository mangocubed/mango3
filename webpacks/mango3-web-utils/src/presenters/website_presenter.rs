use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;

use super::{BlobPresenter, HashtagPresenter};

#[cfg(feature = "ssr")]
use super::FromModel;

#[cfg(feature = "website-presenter")]
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

#[cfg(feature = "website-presenter")]
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

#[cfg(all(feature = "ssr", feature = "website-presenter"))]
impl FromModel<Website> for WebsitePresenter {
    async fn from_model(website: &Website) -> Self {
        #[allow(unused_variables)]
        let core_context = crate::ssr::expect_core_context();
        let hashtags = futures::future::join_all(
            website
                .hashtags()
                .await
                .iter()
                .map(|hashtag| HashtagPresenter::from_model(hashtag)),
        )
        .await;
        let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob().await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        };
        let cover_image_blob = if let Some(Ok(blob)) = website.cover_image_blob().await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        };

        #[cfg(feature = "website-storage")]
        let (available_storage, max_storage, used_storage) = (
            website.available_storage(&core_context).await,
            website.max_storage(),
            website.used_storage(&core_context).await,
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

#[cfg(feature = "website-min-presenter")]
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

#[cfg(feature = "website-min-presenter")]
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

#[cfg(all(feature = "ssr", feature = "website-min-presenter"))]
impl FromModel<Website> for WebsiteMinPresenter {
    async fn from_model(website: &Website) -> Self {
        let hashtags = futures::future::join_all(
            website
                .hashtags()
                .await
                .iter()
                .map(|hashtag| HashtagPresenter::from_model(hashtag)),
        )
        .await;
        let icon_image_blob = if let Some(Ok(blob)) = website.icon_image_blob().await {
            Some(BlobPresenter::from_model(&blob).await)
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

#[cfg(all(feature = "website-min-presenter", feature = "website-presenter"))]
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

#[cfg(all(feature = "website-min-presenter", feature = "website-presenter"))]
impl From<WebsitePresenter> for WebsiteMinPresenter {
    fn from(website: WebsitePresenter) -> Self {
        Self::from(&website)
    }
}

#[cfg(feature = "ssr")]
impl FromModel<Website> for () {
    async fn from_model(_: &Website) -> Self {
        ()
    }
}
