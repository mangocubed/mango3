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
    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        query_as!(
            Self,
            r#"SELECT
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at
            FROM confirmation_codes WHERE id = $1 LIMIT 1"#,
            id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_user(
        core_context: &CoreContext,
        user: &User,
        action: ConfirmationCodeAction,
    ) -> sqlx::Result<Self> {
        query_as!(
            Self,
            r#"SELECT
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at
            FROM confirmation_codes WHERE user_id = $1 AND action = $2 LIMIT 1"#,
            user.id,                          // $1
            action as ConfirmationCodeAction, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }

    async fn verify_code(&self, core_context: &CoreContext, code: &str) -> bool {
        if self.failed_attempts < 3 && verify_password(code, &self.encrypted_code) {
            return true;
        }

        let _ = query!(
            "UPDATE confirmation_codes SET failed_attempts = failed_attempts + 1 WHERE id = $1",
            self.id
        )
        .execute(&core_context.db_pool)
        .await;

        false
    }
}
