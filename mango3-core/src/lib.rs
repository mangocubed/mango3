use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[cfg(test)]
mod test_utils;

pub mod config;
pub mod constants;
pub mod enums;
pub mod jobs;
pub mod locales;
pub mod models;
pub mod pagination;
pub mod validator;

use config::DATABASE_CONFIG;
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
