use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::config::MISC_CONFIG;
use crate::enums::MailerJobCommand;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::{encrypt_password, generate_random_string, verify_password, User};

pub struct ConfirmationCode {
    pub id: Uuid,
    encrypted_code: String,
    pub failed_attempts: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl ConfirmationCode {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM confirmation_codes WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        query_as!(Self, "SELECT * FROM confirmation_codes WHERE id = $1 LIMIT 1", id,)
            .fetch_one(&core_context.db_pool)
            .await
    }

    pub async fn insert(core_context: &CoreContext, user: &User, action: &str) -> Result<Self, ValidationErrors> {
        let code = generate_random_string(MISC_CONFIG.confirmation_code_length);

        let encrypted_code = encrypt_password(&code);

        let result = query_as!(
            Self,
            "INSERT INTO confirmation_codes (encrypted_code) VALUES ($1) RETURNING *",
            encrypted_code
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(confirmation_code) => {
                core_context
                    .jobs
                    .mailer(
                        user,
                        MailerJobCommand::ConfirmationCode {
                            action: action.to_owned(),
                            code,
                        },
                    )
                    .await;

                Ok(confirmation_code)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn verify_code(&self, core_context: &CoreContext, code: &str) -> bool {
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
