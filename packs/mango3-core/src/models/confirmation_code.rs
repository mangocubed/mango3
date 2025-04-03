use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::ConfirmationCodeAction;
use crate::CoreContext;

use super::User;

pub struct ConfirmationCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: ConfirmationCodeAction,
    pub(crate) encrypted_code: String,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ConfirmationCode {
    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }
}
