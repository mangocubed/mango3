use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub mod commands;
pub mod config;
pub mod constants;
pub mod enums;
pub mod models;
pub mod utils;

#[cfg(test)]
mod test_utils;

use config::DATABASE_CONFIG;

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
    #[cfg(feature = "jobs")]
    pub jobs: utils::Jobs,
}

impl CoreContext {
    pub async fn setup() -> Self {
        Self {
            db_pool: setup_db_pool().await,
            #[cfg(feature = "jobs")]
            jobs: utils::Jobs::setup().await,
        }
    }
}
