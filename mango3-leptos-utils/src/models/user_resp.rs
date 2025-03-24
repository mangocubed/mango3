use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

use mango3_utils::models::Hashtag;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobResp, UserProfileResp};

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPreviewResp {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub hashtags: Vec<Hashtag>,
    pub avatar_image_blob: Option<BlobResp>,
    pub text_avatar_url: String,
    pub url: String,
    pub role: String,
    pub is_disabled: bool,

    #[cfg(feature = "user_bio_preview_html")]
    pub bio_preview_html: String,
}

impl UserPreviewResp {
    pub fn avatar_image_url(&self, size: u16) -> String {
        if self.avatar_image_blob.is_none() || self.is_disabled {
            return format!("{}?size={}", self.text_avatar_url, size);
        }

        self.avatar_image_blob
            .as_ref()
            .expect("Could not get avatar image blob")
            .variant_url(size, size, true)
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<User> for UserPreviewResp {
    async fn from_core(core_context: &CoreContext, user: &User) -> Self {
        let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };

        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
            hashtags: user.hashtags(&core_context).await,
            avatar_image_blob,
            text_avatar_url: user.text_avatar_url().to_string(),
            url: user.url().to_string(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),

            #[cfg(feature = "user_bio_preview_html")]
            bio_preview_html: user.bio_preview_html().await,
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
            hashtags: value.hashtags,
            avatar_image_blob: value.avatar_image_blob,
            url: value.url,
            text_avatar_url: value.text_avatar_url,
            role: value.role,
            is_disabled: value.is_disabled,

            #[cfg(feature = "user_bio_preview_html")]
            bio_preview_html: value.bio_preview_html,
        }
    }
}

impl From<UserProfileResp> for UserPreviewResp {
    fn from(value: UserProfileResp) -> Self {
        Self {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
            initials: value.initials,
            hashtags: value.hashtags,
            avatar_image_blob: value.avatar_image_blob,
            url: value.url,
            text_avatar_url: value.text_avatar_url,
            role: value.role,
            is_disabled: value.is_disabled,

            #[cfg(feature = "user_bio_preview_html")]
            bio_preview_html: value.bio_preview_html,
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

    #[cfg(feature = "user_bio_preview_html")]
    pub bio_preview_html: String,

    pub hashtags: Vec<Hashtag>,
    pub avatar_image_blob: Option<BlobResp>,
    pub can_insert_website: bool,
    pub url: String,
    pub text_avatar_url: String,
    pub role: String,
    pub is_disabled: bool,
}

impl UserResp {
    pub fn avatar_image_url(&self, size: u16) -> String {
        if self.avatar_image_blob.is_none() || self.is_disabled {
            return format!("{}?size={}", self.text_avatar_url, size);
        }

        self.avatar_image_blob
            .as_ref()
            .expect("Could not get avatar image blob")
            .variant_url(size, size, true)
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<User> for UserResp {
    async fn from_core(core_context: &CoreContext, user: &User) -> Self {
        let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };

        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
            email: user.email.clone(),
            email_is_confirmed: user.email_is_confirmed(),
            hashtags: user.hashtags(&core_context).await,
            avatar_image_blob,
            can_insert_website: user.can_insert_website(&core_context).await,
            url: user.url().to_string(),
            text_avatar_url: user.text_avatar_url().to_string(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),

            #[cfg(feature = "user_bio_preview_html")]
            bio_preview_html: user.bio_preview_html().await,
        }
    }
}
