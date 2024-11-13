use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserProfileResp {
    id: String,
    pub username: String,
    pub display_name: String,
    pub full_name: String,
    pub initials: String,
    pub birthdate: String,
    pub country_alpha2: String,
    pub country_name: String,
    pub bio: String,
    pub avatar_image_blob: Option<BlobResp>,
}

#[cfg(feature = "ssr")]
impl UserProfileResp {
    pub async fn from_user(core_context: &CoreContext, user: User) -> Self {
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
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
        }
    }
}
