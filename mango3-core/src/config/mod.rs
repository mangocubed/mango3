use std::fs;
use std::sync::LazyLock;

use dotenvy::dotenv;
use figment::providers::{Env, Serialized};
use figment::Figment;
use serde::{Deserialize, Serialize};

mod basic_config;
mod cache_config;
mod mailer_config;
mod misc_config;
mod user_config;

#[cfg(feature = "website_storage")]
mod website_config;

pub use basic_config::BasicConfig;
pub(crate) use cache_config::CacheConfig;
pub use mailer_config::MailerConfig;
pub use misc_config::MiscConfig;
pub(crate) use user_config::UserConfig;

#[cfg(feature = "website_storage")]
pub(crate) use website_config::WebsiteConfig;

pub static BASIC_CONFIG: LazyLock<BasicConfig> = LazyLock::new(BasicConfig::load);
pub(crate) static CACHE_CONFIG: LazyLock<CacheConfig> = LazyLock::new(CacheConfig::load);
pub(crate) static DATABASE_CONFIG: LazyLock<DatabaseConfig> = LazyLock::new(DatabaseConfig::load);
pub(crate) static JOBS_CONFIG: LazyLock<JobsConfig> = LazyLock::new(JobsConfig::load);
pub static MAILER_CONFIG: LazyLock<MailerConfig> = LazyLock::new(MailerConfig::load);
pub static MISC_CONFIG: LazyLock<MiscConfig> = LazyLock::new(MiscConfig::load);
pub static SESSIONS_CONFIG: LazyLock<SessionsConfig> = LazyLock::new(SessionsConfig::load);
pub static USER_CONFIG: LazyLock<UserConfig> = LazyLock::new(UserConfig::load);

#[cfg(feature = "website_storage")]
pub(crate) static WEBSITE_CONFIG: LazyLock<WebsiteConfig> = LazyLock::new(WebsiteConfig::load);

pub fn load_config() {
    let _ = dotenv();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let _ = fs::create_dir_all(MISC_CONFIG.storage_tmp_path());
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

#[derive(Deserialize, Serialize)]
pub(crate) struct DatabaseConfig {
    pub(crate) max_connections: u8,
    pub(crate) url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let db_suffix = if cfg!(test) { "test" } else { "dev" };

        Self {
            max_connections: 5,
            url: format!("postgres://mango3:mango3@127.0.0.1:5432/mango3_{db_suffix}"),
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
        let db_number = if cfg!(test) { "10" } else { "0" };

        Self {
            redis_url: format!("redis://127.0.0.1:6379/{db_number}"),
        }
    }
}

impl JobsConfig {
    fn load() -> Self {
        extract_from_env("JOBS_")
    }
}

#[derive(Deserialize, Serialize)]
pub struct SessionsConfig {
    pub key: String,
    pub redis_url: String,
}

impl Default for SessionsConfig {
    fn default() -> Self {
        let db_number = if cfg!(test) { "11" } else { "1" };

        Self {
            key: "abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX".to_owned(),
            redis_url: format!("redis://127.0.0.1:6379/{db_number}"),
        }
    }
}

impl SessionsConfig {
    fn load() -> Self {
        extract_from_env("SESSIONS_")
    }
}
