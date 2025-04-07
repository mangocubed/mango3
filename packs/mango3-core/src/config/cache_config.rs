use serde::{Deserialize, Serialize};

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub(crate) struct CacheConfig {
    pub(crate) redis_url: String,
    pub(crate) ttl: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        let db_number = if cfg!(test) { "12" } else { "2" };

        Self {
            redis_url: format!("redis://127.0.0.1:6379/{db_number}"),
            ttl: 3600,
        }
    }
}

impl CacheConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("CACHE_")
    }
}
