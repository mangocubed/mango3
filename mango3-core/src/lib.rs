use regex::Match;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod commands;
pub mod config;
pub mod constants;
pub mod enums;
pub mod info;
pub mod jobs;
pub mod models;
pub mod pagination;
pub mod utils;

#[cfg(feature = "locales")]
pub mod locales;
#[cfg(feature = "validator")]
pub mod validator;

#[cfg(test)]
mod test_utils;

use config::DATABASE_CONFIG;
use constants::HASHTAG_LOOKAROUND;
use jobs::Jobs;

type DBPool = PgPool;

async fn setup_db_pool() -> DBPool {
    PgPoolOptions::new()
        .max_connections(DATABASE_CONFIG.max_connections as u32)
        .connect(&DATABASE_CONFIG.url)
        .await
        .expect("Failed to create DB pool.")
}

#[derive(Clone)]
pub struct CoreContext {
    db_pool: DBPool,
    pub jobs: Jobs,
}

impl CoreContext {
    pub async fn setup() -> Self {
        Self {
            db_pool: setup_db_pool().await,
            jobs: Jobs::setup().await,
        }
    }
}

pub fn hashtag_has_lookaround(content: &str, match_: Match) -> bool {
    (match_.start() == 1 || HASHTAG_LOOKAROUND.contains(&content.get(match_.start() - 2..match_.start() - 1)))
        && HASHTAG_LOOKAROUND.contains(&content.get(match_.end()..match_.end() + 1))
}
