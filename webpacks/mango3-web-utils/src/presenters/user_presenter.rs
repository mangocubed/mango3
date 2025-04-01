use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use std::future::Future;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobPresenter, HashtagPresenter};

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPresenter {
    id: Uuid,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub email: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub avatar_image_blob: Option<BlobPresenter>,
    pub can_insert_website: bool,
    pub url: Url,
    pub text_avatar_url: Url,
    pub role: String,
    pub is_disabled: bool,

    #[cfg(feature = "user-bio-preview-html")]
    pub bio_preview_html: String,
    #[cfg(feature = "user-email-is-confirmed")]
    pub email_is_confirmed: bool,
}

impl UserPresenter {
    pub fn avatar_image_url(&self, size: u16) -> Url {
        if self.avatar_image_blob.is_none() || self.is_disabled {
            let mut text_avatar_url = self.text_avatar_url.clone();
            text_avatar_url.set_query(Some(&format!("size={size}")));
            return text_avatar_url;
        }

        self.avatar_image_blob
            .as_ref()
            .expect("Could not get avatar image blob")
            .variant_url(size, size, true)
    }
}

#[cfg(feature = "ssr")]
impl FromModel<User> for UserPresenter {
    fn from_model(core_context: &CoreContext, user: &User) -> impl Future<Output = Self> {
        async move {
            let hashtags = futures::future::join_all(
                user.hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };

            Self {
                id: user.id,
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                initials: user.initials(),
                email: user.email.clone(),
                hashtags,
                avatar_image_blob,
                can_insert_website: user.can_insert_website(&core_context).await,
                url: user.url(),
                text_avatar_url: user.text_avatar_url(),
                role: user.role.to_string(),
                is_disabled: user.is_disabled(),

                #[cfg(feature = "user-bio-preview-html")]
                bio_preview_html: user.bio_preview_html().await,
                #[cfg(feature = "user-email-is-confirmed")]
                email_is_confirmed: user.email_is_confirmed(),
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserMinPresenter {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub avatar_image_blob: Option<BlobPresenter>,
    pub text_avatar_url: Url,
    pub url: Url,
    pub role: String,
    pub is_disabled: bool,

    #[cfg(feature = "user-bio-preview-html")]
    pub bio_preview_html: String,
}

impl UserMinPresenter {
    pub fn avatar_image_url(&self, size: u16) -> Url {
        if self.avatar_image_blob.is_none() || self.is_disabled {
            let mut text_avatar_url = self.text_avatar_url.clone();
            text_avatar_url.set_query(Some(&format!("size={size}")));
            return text_avatar_url;
        }

        self.avatar_image_blob
            .as_ref()
            .expect("Could not get avatar image blob")
            .variant_url(size, size, true)
    }
}

#[cfg(feature = "ssr")]
impl FromModel<User> for UserMinPresenter {
    fn from_model(core_context: &CoreContext, user: &User) -> impl Future<Output = Self> {
        async move {
            let hashtags = futures::future::join_all(
                user.hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };

            UserMinPresenter {
                id: user.id,
                username: user.username.clone(),
                display_name: user.display_name.clone(),
                initials: user.initials(),
                hashtags,
                avatar_image_blob,
                text_avatar_url: user.text_avatar_url(),
                url: user.url(),
                role: user.role.to_string(),
                is_disabled: user.is_disabled(),

                #[cfg(feature = "user-bio-preview-html")]
                bio_preview_html: user.bio_preview_html().await,
            }
        }
    }
}

impl From<UserPresenter> for UserMinPresenter {
    fn from(value: UserPresenter) -> Self {
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

            #[cfg(feature = "user-bio-preview-html")]
            bio_preview_html: value.bio_preview_html,
        }
    }
}
