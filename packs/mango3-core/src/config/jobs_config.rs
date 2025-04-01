use serde::{Deserialize, Serialize};

use super::extract_from_env;

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
    pub(crate) fn load() -> Self {
        extract_from_env("JOBS_")
    }
}
