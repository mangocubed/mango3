use serde::{Deserialize, Serialize};

#[cfg(feature = "website_storage")]
use size::Size;

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub(crate) struct WebsiteConfig {
    #[cfg(feature = "website_storage")]
    pub(crate) max_storage: Size,
}

impl Default for WebsiteConfig {
    fn default() -> Self {
        Self {
            #[cfg(feature = "website_storage")]
            max_storage: Size::from_gib(1),
        }
    }
}

impl WebsiteConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("WEBSITE_")
    }

    // pub fn max_storage(&self) -> Size {
    //     self.max_storage.parse().expect("Could not get max storage size")
    // }
}
