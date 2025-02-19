use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobResp, HashtagResp};

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPreviewResp {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub bio_preview_html: String,
    pub hashtags: Vec<HashtagResp>,
    pub avatar_image_blob: Option<BlobResp>,
    pub url: String,
    pub text_avatar_url: String,
    pub role: String,
    pub is_disabled: bool,
}

impl UserPreviewResp {
    pub fn avatar_image_url(&self, size: u16) -> String {
        self.avatar_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| format!("{}?size={}", self.text_avatar_url, size))
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<User> for UserPreviewResp {
    async fn from_core(core_context: &CoreContext, user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
            hashtags: user
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            bio_preview_html: user.bio_preview_html().await,
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            url: user.url().to_string(),
            text_avatar_url: user.text_avatar_url().to_string(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),
        }
    }
}

impl From<UserResp> for UserPreviewResp {
    fn from(value: UserResp) -> Self {
        Self {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
            initials: value.initials,
            bio_preview_html: value.bio_preview_html,
            hashtags: value.hashtags,
            avatar_image_blob: value.avatar_image_blob,
            url: value.url,
            text_avatar_url: value.text_avatar_url,
            role: value.role,
            is_disabled: value.is_disabled,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserResp {
    id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub email: String,
    pub email_is_confirmed: bool,
    pub bio_preview_html: String,
    pub hashtags: Vec<HashtagResp>,
    pub avatar_image_blob: Option<BlobResp>,
    pub can_insert_website: bool,
    pub url: String,
    pub text_avatar_url: String,
    pub role: String,
    pub is_disabled: bool,
}

impl UserResp {
    pub fn avatar_image_url(&self, size: u16) -> String {
        self.avatar_image_blob
            .as_ref()
            .map(|blob| blob.variant_url(size, size, true))
            .unwrap_or_else(|| format!("{}?size={}", self.text_avatar_url, size))
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<User> for UserResp {
    async fn from_core(core_context: &CoreContext, user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
            email: user.email.clone(),
            email_is_confirmed: user.email_is_confirmed(),
            bio_preview_html: user.bio_preview_html().await,
            hashtags: user
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            can_insert_website: user.can_insert_website(&core_context).await,
            url: user.url().to_string(),
            text_avatar_url: user.text_avatar_url().to_string(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),
        }
    }
}
