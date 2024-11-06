use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::User;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserResp {
    id: String,
    pub username: String,
    pub display_name: String,
    pub initials: String,
}

#[cfg(feature = "ssr")]
impl UserResp {
    pub async fn from_user(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            initials: user.initials(),
        }
    }
}
