use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use mango3_web_utils::presenters::{BlobPresenter, HashtagPresenter, UserMinPresenter};

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
impl ToPresenter<UserProfilePresenter> for User {
    fn to_presenter(&self, core_context: &CoreContext) -> impl Future<Output = UserProfilePresenter> {
        let avatar_image_blob = if let Some(Ok(blob)) = self.avatar_image_blob(&core_context).await {
            Some(blob.to_presenter(core_context).await)
        } else {
            None
        };

        Self {
            id: self.id,
            username: self.username.clone(),
            display_name: self.display_name.clone(),
            full_name: self.full_name.clone(),
            initials: self.initials(),
            birthdate: self.birthdate.to_string(),
            country_alpha2: self.country_alpha2.clone(),
            country_name: self.country().name.to_owned(),
            bio_html: self.bio_html().await,
            hashtags: self.hashtags(&core_context).await,
            avatar_image_blob,
            text_avatar_url: self.text_avatar_url(),
            url: self.url(),
            role: self.role.to_string(),
            is_disabled: self.is_disabled(),
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
            hashtags: value.hashtags,
            avatar_image_blob: value.avatar_image_blob,
            url: value.url,
            text_avatar_url: value.text_avatar_url,
            role: value.role,
            is_disabled: value.is_disabled,
        }
    }
}
