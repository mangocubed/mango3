use chrono::{DateTime, Utc};
use log::info;

use mango3_core::commands::{delete_all_expired_confirmation_codes, delete_orphaned_blobs};

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Reminder(DateTime<Utc>);

impl From<DateTime<Utc>> for Reminder {
    fn from(t: DateTime<Utc>) -> Self {
        Reminder(t)
    }
}

pub async fn scheduled_worker(reminder: Reminder) {
    info!("Running scheduled worker at {reminder:?}");

    info!("Deleting all expired confirmation codes...");
    let _ = delete_all_expired_confirmation_codes().await;

    info!("Deleting all orphaned blobs...");
    let _ = delete_orphaned_blobs().await;

    info!("Done!");
}
