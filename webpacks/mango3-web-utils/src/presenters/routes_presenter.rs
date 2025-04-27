use serde::{Deserialize, Serialize};
use url::Url;

#[cfg(feature = "server")]
use mango3_core::config::BasicConfig;

#[derive(Clone, Deserialize, Serialize)]
pub struct RoutesPresenter {
    pub about_url: Option<Url>,
    assets_url: Url,
    pub home_url: Url,
    pub login_url: Url,
    pub my_account_url: Url,
    pub new_website_url: Url,
    pub privacy_policy_url: Option<Url>,
    pub register_url: Url,
    pub studio_url: Url,
    pub terms_of_service_url: Option<Url>,
}

impl Default for RoutesPresenter {
    fn default() -> Self {
        let url = Url::parse("a://a").unwrap();

        Self {
            about_url: None,
            assets_url: url.clone(),
            home_url: url.clone(),
            login_url: url.clone(),
            my_account_url: url.clone(),
            new_website_url: url.clone(),
            privacy_policy_url: None,
            register_url: url.clone(),
            studio_url: url,
            terms_of_service_url: None,
        }
    }
}

#[cfg(feature = "server")]
impl From<BasicConfig> for RoutesPresenter {
    fn from(basic_config: BasicConfig) -> Self {
        Self {
            about_url: basic_config.about_url.clone(),
            assets_url: basic_config.assets_url().clone(),
            home_url: basic_config.home_url().clone(),
            login_url: basic_config.login_url(),
            my_account_url: basic_config.my_account_url(),
            new_website_url: basic_config.new_website_url(),
            privacy_policy_url: basic_config.privacy_policy_url.clone(),
            register_url: basic_config.register_url(),
            studio_url: basic_config.studio_url(),
            terms_of_service_url: basic_config.terms_of_service_url.clone(),
        }
    }
}

impl RoutesPresenter {
    pub fn asset_url(&self, file_name: &str) -> Url {
        self.assets_url.join(file_name).unwrap()
    }
}
