use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::config::BasicConfig;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct BasicConfigResp {
    pub about_url: String,
    assets_url: String,
    pub copyright: String,
    pub description: String,
    pub domain: String,
    pub enable_register: bool,
    pub home_url: String,
    pub login_url: String,
    pub my_account_url: String,
    pub new_website_url: String,
    pub privacy_policy_url: String,
    pub register_url: String,
    pub studio_url: String,
    pub support_email_address: String,
    pub terms_of_service_url: String,
    pub title: String,
}

#[cfg(feature = "ssr")]
impl From<BasicConfig> for BasicConfigResp {
    fn from(basic_config: BasicConfig) -> Self {
        Self {
            about_url: basic_config.about_url.clone(),
            assets_url: basic_config.assets_url().to_string(),
            copyright: basic_config.copyright.clone(),
            description: basic_config.description.clone(),
            domain: basic_config.domain.to_string(),
            enable_register: basic_config.enable_register,
            home_url: basic_config.home_url().to_string(),
            login_url: basic_config.login_url().to_string(),
            my_account_url: basic_config.my_account_url().to_string(),
            new_website_url: basic_config.new_website_url().to_string(),
            privacy_policy_url: basic_config.privacy_policy_url.clone(),
            register_url: basic_config.register_url().to_string(),
            studio_url: basic_config.studio_url().to_string(),
            support_email_address: basic_config.support_email_address,
            terms_of_service_url: basic_config.terms_of_service_url.clone(),
            title: basic_config.title,
        }
    }
}

impl BasicConfigResp {
    pub fn asset_url(&self, file_name: &str) -> String {
        format!("{}{}", self.assets_url, file_name)
    }
}
