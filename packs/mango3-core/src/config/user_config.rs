use serde::{Deserialize, Serialize};

#[cfg(feature = "insert-user")]
use crate::enums::UserRole;

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct UserConfig {
    pub default_disabled: bool,
    #[cfg(feature = "insert-user")]
    default_role: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            default_disabled: false,
            #[cfg(feature = "insert-user")]
            default_role: "user".to_owned(),
        }
    }
}

impl UserConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("USER_")
    }

    #[cfg(feature = "insert-user")]
    pub(crate) fn default_role(&self) -> UserRole {
        (&self.default_role).into()
    }
}
