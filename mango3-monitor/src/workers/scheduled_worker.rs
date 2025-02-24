use chrono::{DateTime, Utc};
use log::info;
use mango3_core::models::{ConfirmationCode, UserPasswordReset, UserSession};
use mango3_core::CoreContext;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Reminder(DateTime<Utc>);

impl From<DateTime<Utc>> for Reminder {
    fn from(t: DateTime<Utc>) -> Self {
        Reminder(t)
    }
}

pub async fn scheduled_worker(reminder: Reminder) {
    info!("Running scheduled worker at {reminder:?}");

    let core_context = CoreContext::setup().await;

    info!("Deleting all expired confirmation codes...");
    let _ = ConfirmationCode::delete_all_expired(&core_context).await;

    info!("Delete all expired user password resets...");
    let _ = UserPasswordReset::delete_all_expired(&core_context).await;

    info!("Deleting all expired (and unconfirmed) user sessions...");
    let _ = UserSession::delete_all_expired(&core_context).await;

    info!("Done!");
}
