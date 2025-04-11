use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::OnceCell;

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

static DB_POOL_CELL: OnceCell<DBPool> = OnceCell::const_new();

#[cfg(feature = "jobs")]
static JOBS_CELL: OnceCell<utils::Jobs> = OnceCell::const_new();

async fn db_pool<'a>() -> &'a DBPool {
    DB_POOL_CELL
        .get_or_init(|| async {
            PgPoolOptions::new()
                .max_connections(DATABASE_CONFIG.max_connections as u32)
                .connect(&DATABASE_CONFIG.url)
                .await
                .expect("Failed to create DB pool.")
        })
        .await
}

#[cfg(feature = "jobs")]
#[allow(dead_code)]
async fn jobs<'a>() -> &'a utils::Jobs {
    JOBS_CELL.get_or_init(|| async { utils::Jobs::setup().await }).await
}

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
