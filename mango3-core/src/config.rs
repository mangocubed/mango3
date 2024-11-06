use dotenvy::dotenv;
use figment::providers::{Env, Serialized};
use figment::Figment;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use url::Url;

lazy_static! {
    pub static ref BASIC_CONFIG: BasicConfig = BasicConfig::load();
    pub(crate) static ref DATABASE_CONFIG: DatabaseConfig = DatabaseConfig::load();
    pub(crate) static ref JOBS_CONFIG: JobsConfig = JobsConfig::load();
    pub static ref MAILER_CONFIG: MailerConfig = MailerConfig::load();
    pub static ref SESSIONS_CONFIG: SessionsConfig = SessionsConfig::load();
}

pub fn load_config() {
    let _ = dotenv();
}

fn extract_from_env<'a, T>(prefix: &str) -> T
where
    T: Deserialize<'a> + Serialize + Default,
{
    Figment::from(Serialized::defaults(T::default()))
        .merge(Env::prefixed(prefix))
        .extract()
        .unwrap()
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BasicConfig {
    pub copyright: String,
    pub domain: String,
    pub secure: bool,
    pub title: String,
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            copyright: "© 2024, Mango³ Team".to_owned(),
            domain: "mango3.localhost".to_owned(),
            secure: false,
            title: "Mango³ Dev".to_owned(),
        }
    }
}

impl BasicConfig {
    fn load() -> Self {
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

    pub fn home_url(&self) -> Url {
        Url::parse(&format!("{}://{}", self.scheme(), self.domain)).unwrap()
    }

    pub fn login_url(&self) -> Url {
        self.accounts_url().join("login").unwrap()
    }

    pub fn my_account_url(&self) -> Url {
        Url::parse(&format!("{}://my-account.{}", self.scheme(), self.domain)).unwrap()
    }

    pub fn register_url(&self) -> Url {
        self.accounts_url().join("register").unwrap()
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DatabaseConfig {
    pub(crate) max_connections: i8,
    pub(crate) url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 5,
            url: "postgres://mango3:mango3@127.0.0.1:5432/mango3_dev".to_owned(),
        }
    }
}

impl DatabaseConfig {
    fn load() -> Self {
        extract_from_env("DATABASE_")
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct JobsConfig {
    pub(crate) redis_url: String,
}

impl Default for JobsConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://127.0.0.1:6379/0".to_owned(),
        }
    }
}

impl JobsConfig {
    fn load() -> Self {
        extract_from_env("JOBS_")
    }
}

#[derive(Deserialize, Serialize)]
pub struct MailerConfig {
    pub enable: bool,
    pub sender_address: String,
    pub smtp_address: String,
    pub smtp_password: String,
    pub smtp_security: String,
    pub smtp_username: String,
}

impl Default for MailerConfig {
    fn default() -> Self {
        Self {
            enable: false,
            sender_address: "Mango³ Dev <no-reply@localhost>".to_owned(),
            smtp_address: "localhost".to_owned(),
            smtp_password: "".to_owned(),
            smtp_security: "none".to_owned(),
            smtp_username: "".to_owned(),
        }
    }
}

impl MailerConfig {
    fn load() -> Self {
        extract_from_env("MAILER_")
    }
}

#[derive(Deserialize, Serialize)]
pub struct SessionsConfig {
    pub key: String,
    pub redis_url: String,
}

impl Default for SessionsConfig {
    fn default() -> Self {
        Self {
            key: "abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX".to_owned(),
            redis_url: "redis://127.0.0.1:6379/1".to_owned(),
        }
    }
}

impl SessionsConfig {
    fn load() -> Self {
        extract_from_env("SESSIONS_")
    }
}
