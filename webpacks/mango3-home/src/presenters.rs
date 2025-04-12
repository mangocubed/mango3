use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use mango3_web_utils::presenters::{BlobPresenter, HashtagPresenter, UserMinPresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserProfilePresenter {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub full_name: String,
    pub initials: String,
    pub birthdate: String,
    pub country_alpha2: String,
    pub country_name: String,
    pub bio_preview_html: String,
    pub bio_html: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub avatar_image_blob: Option<BlobPresenter>,
    pub text_avatar_url: Url,
    pub url: Url,
    pub role: String,
    pub is_disabled: bool,
}

impl UserProfilePresenter {
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
impl FromModel<User> for UserProfilePresenter {
    async fn from_model(user: &User) -> Self {
        let hashtags = futures::future::join_all(
            user.hashtags()
                .await
                .iter()
                .map(|hashtag| HashtagPresenter::from_model(hashtag)),
        )
        .await;
        let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob().await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        };

        Self {
            id: user.id,
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            full_name: user.full_name.clone(),
            initials: user.initials(),
            birthdate: user.birthdate.to_string(),
            country_alpha2: user.country_alpha2.clone(),
            country_name: user.country().name.to_owned(),
            bio_preview_html: user.bio_preview_html().await,
            bio_html: user.bio_html().await,
            hashtags,
            avatar_image_blob,
            text_avatar_url: user.text_avatar_url(),
            url: user.url(),
            role: user.role.to_string(),
            is_disabled: user.is_disabled(),
        }
    }
}

impl From<UserProfilePresenter> for UserMinPresenter {
    fn from(value: UserProfilePresenter) -> Self {
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
