use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(any(feature = "ssr", feature = "server"))]
use mango3_core::config::BasicConfig;

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicConfigPresenter {
    pub about_url: Option<Url>,
    assets_url: Url,
    pub copyright: String,
    pub description: String,
    pub domain: String,
    pub enable_register: bool,
    pub home_url: Url,
    pub login_url: Url,
    pub my_account_url: Url,
    pub new_website_url: Url,
    pub privacy_policy_url: Option<Url>,
    pub register_url: Url,
    pub studio_url: Url,
    pub support_email_address: String,
    pub terms_of_service_url: Option<Url>,
    pub title: String,
}

impl Default for BasicConfigPresenter {
    fn default() -> Self {
        let home_url = Url::parse("a://a").unwrap();

        Self {
            about_url: None,
            assets_url: home_url.clone(),
            copyright: String::new(),
            description: String::new(),
            domain: String::new(),
            enable_register: true,
            home_url: home_url.clone(),
            login_url: home_url.clone(),
            my_account_url: home_url.clone(),
            new_website_url: home_url.clone(),
            privacy_policy_url: None,
            register_url: home_url.clone(),
            studio_url: home_url,
            support_email_address: String::new(),
            terms_of_service_url: None,
            title: String::new(),
        }
    }
}

#[cfg(any(feature = "ssr", feature = "server"))]
impl From<BasicConfig> for BasicConfigPresenter {
    fn from(basic_config: BasicConfig) -> Self {
        Self {
            about_url: basic_config.about_url.clone(),
            assets_url: basic_config.assets_url().clone(),
            copyright: basic_config.copyright.clone(),
            description: basic_config.description.clone(),
            domain: basic_config.domain.clone(),
            enable_register: basic_config.enable_register,
            home_url: basic_config.home_url().clone(),
            login_url: basic_config.login_url(),
            my_account_url: basic_config.my_account_url(),
            new_website_url: basic_config.new_website_url(),
            privacy_policy_url: basic_config.privacy_policy_url.clone(),
            register_url: basic_config.register_url(),
            studio_url: basic_config.studio_url(),
            support_email_address: basic_config.support_email_address,
            terms_of_service_url: basic_config.terms_of_service_url.clone(),
            title: basic_config.title,
        }
    }
}

impl BasicConfigPresenter {
    pub fn asset_url(&self, file_name: &str) -> Url {
        self.assets_url.join(file_name).unwrap()
    }
}
