use apalis::prelude::{Job, Storage};
use apalis::redis::RedisStorage;
use serde::{Deserialize, Serialize};

use crate::config::JOBS_CONFIG;
use crate::enums::{GuestMailerJobCommand, MailerJobCommand};
use crate::models::User;

#[derive(Clone, Debug)]
pub struct Jobs {
    pub storage_guest_mailer: RedisStorage<GuestMailerJob>,
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
            storage_guest_mailer: Self::storage().await,
            storage_mailer: Self::storage().await,
        }
    }

    pub async fn guest_mailer(&self, to: &str, command: GuestMailerJobCommand) {
        self.storage_guest_mailer
            .clone()
            .push(GuestMailerJob {
                to: to.to_owned(),
                command,
            })
            .await
            .expect("Could not store job");
    }

    pub async fn mailer(&self, user: &User, command: MailerJobCommand) {
        self.storage_mailer
            .clone()
            .push(MailerJob {
                user: user.clone(),
                command,
            })
            .await
            .expect("Could not store job");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuestMailerJob {
    pub to: String,
    pub command: GuestMailerJobCommand,
}

impl Job for GuestMailerJob {
    const NAME: &'static str = "GuestMailerJob";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailerJob {
    pub user: User,
    pub command: MailerJobCommand,
}

impl Job for MailerJob {
    const NAME: &'static str = "MailerJob";
}
