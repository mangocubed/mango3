use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserResp {
    id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub email: String,
    pub email_is_confirmed: bool,
    pub avatar_image_blob: Option<BlobResp>,
}

#[cfg(feature = "ssr")]
impl UserResp {
    pub async fn from_user(core_context: &CoreContext, user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
            email: user.email.clone(),
            email_is_confirmed: user.email_is_confirmed(),
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
        }
    }
}
