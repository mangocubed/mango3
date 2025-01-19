use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPreviewResp {
    id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
    pub avatar_image_blob: Option<BlobResp>,
    pub is_creator: bool,
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
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_creator: user.is_creator(),
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
            avatar_image_blob: value.avatar_image_blob,
            is_creator: value.is_creator,
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
    pub avatar_image_blob: Option<BlobResp>,
    pub is_creator: bool,
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
            avatar_image_blob: user
                .avatar_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_creator: user.is_creator(),
        }
    }
}
