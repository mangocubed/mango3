use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::UserSession;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UserSessionResp {
    pub is_confirmed: bool,
}

#[cfg(feature = "ssr")]
impl From<&UserSession> for UserSessionResp {
    fn from(value: &UserSession) -> Self {
        UserSessionResp {
            is_confirmed: value.is_confirmed(),
        }
    }
}
