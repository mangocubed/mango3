use serde::{Deserialize, Serialize};
use uuid::Uuid;

use mango3_web_utils::presenters::BlobPresenter;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct EditUserProfilePresenter {
    pub id: Uuid,
    pub display_name: String,
    pub full_name: String,
    pub birthdate: String,
    pub country_alpha2: String,
    pub bio: String,
    pub avatar_image_blob: Option<BlobPresenter>,
}

#[cfg(feature = "ssr")]
impl FromModel<User> for EditUserProfilePresenter {
    async fn from_model(user: &User) -> Self {
        let core_context = mango3_web_utils::ssr::expect_core_context();
        let avatar_image_blob = if let Some(Ok(blob)) = user.avatar_image_blob(&core_context).await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        };

        Self {
            id: user.id,
            display_name: user.display_name.clone(),
            full_name: user.full_name.clone(),
            birthdate: user.birthdate.to_string(),
            country_alpha2: user.country_alpha2.clone(),
            bio: user.bio.clone(),
            avatar_image_blob,
        }
    }
}
