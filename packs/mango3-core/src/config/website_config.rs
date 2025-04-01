use serde::{Deserialize, Serialize};

use size::Size;

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub(crate) struct WebsiteConfig {
    #[cfg(feature = "website-storage")]
    pub(crate) max_storage: Size,
}

impl Default for WebsiteConfig {
    fn default() -> Self {
        Self {
            max_storage: Size::from_gib(1),
        }
    }
}

impl WebsiteConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("WEBSITE_")
    }
}
