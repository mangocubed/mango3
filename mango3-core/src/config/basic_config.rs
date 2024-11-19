use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::extract_from_env;

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicConfig {
    pub copyright: String,
    pub domain: String,
    pub enable_register: bool,
    pub secure: bool,
    pub title: String,
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            copyright: "© 2024, Mango³ Team".to_owned(),
            domain: "mango3.localhost".to_owned(),
            enable_register: true,
            secure: false,
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

    fn accounts_url(&self) -> Url {
        Url::parse(&format!("{}://accounts.{}", self.scheme(), self.domain)).unwrap()
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
        Url::parse(&format!("{}://my-account.{}", self.scheme(), self.domain)).unwrap()
    }

    pub fn new_website_url(&self) -> Url {
        self.studio_url().join("new-website").unwrap()
    }

    pub fn register_url(&self) -> Url {
        self.accounts_url().join("register").unwrap()
    }

    pub fn studio_url(&self) -> Url {
        Url::parse(&format!("{}://studio.{}", self.scheme(), self.domain)).unwrap()
    }

    fn uploads_url(&self) -> Url {
        Url::parse(&format!("{}://uploads.{}", self.scheme(), self.domain)).unwrap()
    }

    pub fn website_url(&self, subdomain: &str) -> Url {
        Url::parse(&format!("{}://{}.{}", self.scheme(), subdomain, self.domain)).unwrap()
    }
}
