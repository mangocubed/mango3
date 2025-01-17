use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::extract_from_env;

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicConfig {
    pub about_url: String,
    pub copyright: String,
    pub domain: String,
    pub enable_register: bool,
    pub privacy_policy_url: String,
    pub secure: bool,
    pub terms_of_service_url: String,
    pub title: String,
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            about_url: String::new(),
            copyright: "© 2025, Mango³ Team".to_owned(),
            domain: "mango3.local".to_owned(),
            enable_register: true,
            privacy_policy_url: String::new(),
            secure: false,
            terms_of_service_url: String::new(),
            title: "Mango³ Dev".to_owned(),
        }
    }
}

impl BasicConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("BASIC_")
    }

    fn scheme(&self) -> &str {
        if self.secure {
            "https"
        } else {
            "http"
        }
    }

    pub fn assets_url(&self) -> Url {
        self.subdomain_url("assets")
    }

    fn accounts_url(&self) -> Url {
        self.subdomain_url("accounts")
    }

    pub fn blob_url(&self, id: Uuid) -> Url {
        self.uploads_url().join(&format!("blobs/{}", id)).unwrap()
    }

    pub fn home_url(&self) -> Url {
        Url::parse(&format!("{}://{}", self.scheme(), self.domain)).unwrap()
    }

    pub fn login_url(&self) -> Url {
        self.accounts_url().join("login").unwrap()
    }

    pub fn my_account_url(&self) -> Url {
        self.subdomain_url("my-account")
    }

    pub fn new_website_url(&self) -> Url {
        self.studio_url().join("new-website").unwrap()
    }

    pub fn register_url(&self) -> Url {
        self.accounts_url().join("register").unwrap()
    }

    pub fn studio_url(&self) -> Url {
        self.subdomain_url("studio")
    }

    pub fn subdomain_url(&self, subdomain: &str) -> Url {
        Url::parse(&format!("{}://{}.{}", self.scheme(), subdomain, self.domain)).unwrap()
    }

    pub fn text_icon_url(&self, text: &str) -> Url {
        self.uploads_url().join(&format!("text-icons/{text}")).unwrap()
    }

    fn uploads_url(&self) -> Url {
        self.subdomain_url("uploads")
    }

    pub fn website_url(&self, subdomain: &str) -> Url {
        self.subdomain_url(subdomain)
    }
}
