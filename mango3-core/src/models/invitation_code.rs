use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::config::MISC_CONFIG;
use crate::constants::REGEX_EMAIL;
use crate::enums::{GuestMailerJobCommand, Input, InputError};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::generate_random_string;

pub struct InvitationCode {
    pub id: Uuid,
    #[allow(dead_code)]
    email: String,
    #[allow(dead_code)]
    code: String,
    #[allow(dead_code)]
    created_at: DateTime<Utc>,
    #[allow(dead_code)]
    updated_at: Option<DateTime<Utc>>,
}

impl InvitationCode {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM invitation_codes WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_by_code(core_context: &CoreContext, code: &str) -> sqlx::Result<Self> {
        query_as!(Self, "SELECT * FROM invitation_codes WHERE code = $1 LIMIT 1", code)
            .fetch_one(&core_context.db_pool)
            .await
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        query_as!(Self, "SELECT * FROM invitation_codes WHERE id = $1 LIMIT 1", id,)
            .fetch_one(&core_context.db_pool)
            .await
    }

    pub async fn insert(core_context: &CoreContext, email: &str) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let email = email.trim().to_lowercase();

        if validator.validate_presence(Input::Email, &email)
            && validator.validate_length(Input::Email, &email, Some(5), Some(255))
            && validator.validate_format(Input::Email, &email, &REGEX_EMAIL)
        {
            let email_exists = query!(
                "SELECT id FROM users WHERE LOWER(email) = $1 LIMIT 1",
                email // $1
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Email, InputError::AlreadyInUse, &|| !email_exists);
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let code = generate_random_string(MISC_CONFIG.invitation_code_length);

        let result = query_as!(
            Self,
            "INSERT INTO invitation_codes (email, code) VALUES ($1, $2) RETURNING *",
            email, // $1
            code,  // $2
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(invitation_code) => {
                core_context
                    .jobs
                    .guest_mailer(&email, GuestMailerJobCommand::InvitationCode(code))
                    .await;

                Ok(invitation_code)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
