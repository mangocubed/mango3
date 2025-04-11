use std::borrow::Cow;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::ConfirmationCodeAction;
use crate::CoreContext;

use super::User;

pub struct ConfirmationCode<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: ConfirmationCodeAction,
    #[allow(dead_code)]
    pub(crate) encrypted_code: Cow<'a, str>,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ConfirmationCode<'_> {
    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }
}
