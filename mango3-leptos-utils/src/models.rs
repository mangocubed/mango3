use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::config::BasicConfig;

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicConfigResp {
    pub copyright: String,
    pub title: String,
}

#[cfg(feature = "ssr")]
impl From<BasicConfig> for BasicConfigResp {
    fn from(basic_config: BasicConfig) -> Self {
        Self {
            copyright: basic_config.copyright.clone(),
            title: basic_config.title,
        }
    }
}
