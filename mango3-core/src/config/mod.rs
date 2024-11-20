use std::fs;

use dotenvy::dotenv;
use figment::providers::{Env, Serialized};
use figment::Figment;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

mod basic_config;
mod mailer_config;
mod misc_config;

pub use basic_config::BasicConfig;
pub use mailer_config::MailerConfig;
pub use misc_config::MiscConfig;

lazy_static! {
    pub static ref BASIC_CONFIG: BasicConfig = BasicConfig::load();
    pub(crate) static ref DATABASE_CONFIG: DatabaseConfig = DatabaseConfig::load();
    pub(crate) static ref JOBS_CONFIG: JobsConfig = JobsConfig::load();
    pub static ref MAILER_CONFIG: MailerConfig = MailerConfig::load();
    pub static ref MISC_CONFIG: MiscConfig = MiscConfig::load();
    pub static ref SESSIONS_CONFIG: SessionsConfig = SessionsConfig::load();
}

pub fn load_config() {
    let _ = dotenv();
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