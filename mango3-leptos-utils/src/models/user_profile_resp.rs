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
pub struct UserProfileResp {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub full_name: String,
    pub initials: String,
    pub birthdate: String,
    pub country_alpha2: String,
    pub country_name: String,
    pub bio: String,
    pub bio_html: String,
    pub bio_preview_html: String,
    pub hashtags: Vec<HashtagResp>,
    pub avatar_image_blob: Option<BlobResp>,
    pub text_avatar_url: String,
    pub url: String,
    pub role: String,
    pub is_disabled: bool,
}

impl UserProfileResp {
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
impl FromCore<User> for UserProfileResp {
    async fn from_core(core_context: &CoreContext, user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            full_name: user.full_name.clone(),
            initials: user.initials(),
            birthdate: user.birthdate.to_string(),
            country_alpha2: user.country_alpha2.clone(),
            country_name: user.country().name.to_owned(),
            bio: user.bio.clone(),
            bio_html: user.bio_html().await,
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
            text_avatar_url: user.text_avatar_url().to_string(),
            url: user.url().to_string(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),
        }
    }
}
