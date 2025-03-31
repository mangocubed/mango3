use apalis::prelude::Storage;
use apalis_redis::RedisStorage;
use serde::{Deserialize, Serialize};

use crate::config::JOBS_CONFIG;
use crate::enums::{AdminMailerJobCommand, GuestMailerJobCommand, MailerJobCommand};
use crate::models::User;

#[derive(Clone, Debug)]
pub struct Jobs {
    pub storage_admin_mailer: RedisStorage<AdminMailerJob>,
    pub storage_guest_mailer: RedisStorage<GuestMailerJob>,
    pub storage_mailer: RedisStorage<MailerJob>,
}

impl Jobs {
    async fn storage<T: Serialize + for<'de> Deserialize<'de>>() -> RedisStorage<T> {
        let conn = apalis_redis::connect(JOBS_CONFIG.redis_url.clone())
            .await
            .expect("Could not connect to Redis Jobs DB");
        RedisStorage::new(conn)
    }

    pub async fn setup() -> Self {
        Self {
            storage_admin_mailer: Self::storage().await,
            storage_guest_mailer: Self::storage().await,
            storage_mailer: Self::storage().await,
        }
    }

    pub async fn admin_mailer(&self, command: AdminMailerJobCommand) {
        self.storage_admin_mailer
            .clone()
            .push(AdminMailerJob { command })
            .await
            .expect("Could not store job");
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
pub struct AdminMailerJob {
    pub command: AdminMailerJobCommand,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuestMailerJob {
    pub to: String,
    pub command: GuestMailerJobCommand,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailerJob {
    pub user: User,
    pub command: MailerJobCommand,
}
