use std::future::IntoFuture;

use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::config::MISC_CONFIG;
use crate::enums::{ConfirmationCodeAction, Input, InputError, MailerJobCommand};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{encrypt_password, generate_random_string, verify_password, User};

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
    pub async fn confirm<F, IF>(
        &self,
        core_context: &CoreContext,
        action: ConfirmationCodeAction,
        code: &str,
        on_success: F,
    ) -> Result<(), ValidationErrors>
    where
        F: Fn() -> IF,
        IF: IntoFuture<Output = Result<(), ValidationErrors>>,
    {
        let mut validator = Validator::default();

        if action != self.action {
            return Err(ValidationErrors::default());
        }

        if validator.validate_presence(Input::Code, code) {
            let code_is_verified = self.verify_code(core_context, code).await;

            validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let result = on_success().await;

        match result {
            Ok(()) => {
                let _ = self.delete(core_context).await;

                Ok(())
            }
            errors => errors,
        }
    }

    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM confirmation_codes WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn delete_all_expired(core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM confirmation_codes WHERE created_at < current_timestamp - INTERVAL '1 hour'")
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

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

    pub async fn insert(
        core_context: &CoreContext,
        user: &User,
        action: ConfirmationCodeAction,
    ) -> Result<Self, ValidationErrors> {
        if let Ok(confirmation_code) = Self::get_by_user(core_context, user, action.clone()).await {
            return Ok(confirmation_code);
        }

        let code = generate_random_string(MISC_CONFIG.confirmation_code_length);

        let encrypted_code = encrypt_password(&code);

        let result = query_as!(
            Self,
            r#"INSERT INTO confirmation_codes (user_id, action, encrypted_code) VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                action as "action!: ConfirmationCodeAction",
                encrypted_code,
                failed_attempts,
                created_at,
                updated_at"#,
            user.id,                                  // $1
            action.clone() as ConfirmationCodeAction, // $2
            encrypted_code                            // $3
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(confirmation_code) => {
                core_context
                    .jobs
                    .mailer(user, MailerJobCommand::ConfirmationCode { action, code })
                    .await;

                Ok(confirmation_code)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        User::get_by_id(core_context, self.user_id).await
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
