use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::config::MISC_CONFIG;
use crate::enums::{ConfirmationCodeAction, MailerJobCommand};
use crate::CoreContext;

use super::{encrypt_password, verify_password, User};

pub struct ConfirmationCode {
    pub id: Uuid,
    user_id: Uuid,
    action: ConfirmationCodeAction,
    encrypted_code: String,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ConfirmationCode {
    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }
}
