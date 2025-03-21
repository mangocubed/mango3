use std::future::Future;

use sqlx::{query, query_as};

use mango3_utils::models::InvitationCode;

use crate::config::MISC_CONFIG;
use crate::constants::REGEX_EMAIL;
use crate::enums::{GuestMailerJobCommand, Input, InputError};
use crate::models::generate_random_string;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

pub trait InvitationCodeInsert {
    fn insert(
        core_context: &CoreContext,
        email: &str,
    ) -> impl Future<Output = Result<InvitationCode, ValidationErrors>>;
}

impl InvitationCodeInsert for InvitationCode {
    fn insert(
        core_context: &CoreContext,
        email: &str,
    ) -> impl Future<Output = Result<InvitationCode, ValidationErrors>> {
        let mut validator = Validator::default();

        let email = email.trim().to_lowercase();

        async move {
            if validator.validate_presence(Input::Email, &email)
                && validator.validate_length(Input::Email, &email, Some(5), Some(256))
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
}
