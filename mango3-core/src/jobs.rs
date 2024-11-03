use apalis::prelude::{Job, Storage};
use apalis::redis::RedisStorage;
use serde::{Deserialize, Serialize};

use crate::config::JOBS_CONFIG;
use crate::enums::MailerJobCommand;

#[derive(Clone, Debug)]
pub struct Jobs {
    pub storage_mailer: RedisStorage<MailerJob>,
}

impl Jobs {
    async fn storage<T: Job + Serialize + for<'de> Deserialize<'de>>() -> RedisStorage<T> {
        let conn = apalis::redis::connect(JOBS_CONFIG.redis_url.clone())
            .await
            .expect("Could not connect to Redis Jobs DB");
        RedisStorage::new(conn)
    }

    pub async fn setup() -> Self {
        Self {
            storage_mailer: Self::storage().await,
        }
    }

    pub async fn mailer(&self, command: MailerJobCommand) {
        self.storage_mailer
            .clone()
            .push(MailerJob { command })
            .await
            .expect("Coult not store job");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailerJob {
    pub command: MailerJobCommand,
}

impl Job for MailerJob {
    const NAME: &'static str = "MailerJob";
}
