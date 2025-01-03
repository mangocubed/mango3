use sqlx::{query, query_as};

use crate::constants::{KEY_TEXT_CONFIRM_YOUR_EMAIL, REGEX_EMAIL};
use crate::enums::{Input, InputError};
use crate::models::ConfirmationCode;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn confirm_email(&self, core_context: &CoreContext, code: &str) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        if validator.validate_presence(Input::Code, code) {
            let email_confirmation_code = self
                .email_confirmation_code(core_context)
                .await
                .ok_or_else(ValidationErrors::default)?
                .map_err(|_| ValidationErrors::default())?;
            let code_is_verified = email_confirmation_code.verify_code(core_context, code).await;

            validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "UPDATE users SET email_confirmation_code_id = NULL, email_confirmed_at = current_timestamp
            WHERE email_confirmed_at IS NULL AND id = $1 RETURNING *",
            self.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    async fn email_confirmation_code(&self, core_context: &CoreContext) -> Option<sqlx::Result<ConfirmationCode>> {
        if let Some(email_confirmation_code_id) = self.email_confirmation_code_id {
            Some(ConfirmationCode::get_by_id(core_context, email_confirmation_code_id).await)
        } else {
            None
        }
    }

    pub fn email_is_confirmed(&self) -> bool {
        self.email_confirmed_at.is_some()
    }

    pub async fn send_email_confirmation_code(&self, core_context: &CoreContext) -> Result<Self, ValidationErrors> {
        if self.email_is_confirmed() {
            return Err(ValidationErrors::default());
        }

        let action = self.i18n().text(KEY_TEXT_CONFIRM_YOUR_EMAIL);

        let confirmation_code = ConfirmationCode::insert(core_context, self, &action).await?;

        query_as!(
            Self,
            "UPDATE users SET email_confirmation_code_id = $2 WHERE id = $1 RETURNING *",
            self.id,              // $1
            confirmation_code.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn update_email(
        &self,
        core_context: &CoreContext,
        email: &str,
        password: &str,
    ) -> Result<Self, ValidationErrors> {
        let email = email.trim().to_lowercase();

        let mut validator = Validator::default();

        if validator.validate_presence(Input::Email, &email)
            && validator.validate_length(Input::Email, &email, Some(5), Some(255))
            && validator.validate_format(Input::Email, &email, &REGEX_EMAIL)
        {
            let email_exists = query!(
                "SELECT id FROM users WHERE id != $1 AND LOWER(email) = $2 LIMIT 1",
                self.id, // $1
                email,   // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Email, InputError::AlreadyInUse, &|| !email_exists);
        }

        if validator.validate_presence(Input::Password, password) {
            validator.custom_validation(Input::Password, InputError::IsInvalid, &|| {
                self.verify_password(password)
            });
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        if self.email == email {
            return Ok(self.clone());
        }

        query_as!(
            Self,
            "UPDATE users SET
                email = $2::text,
                email_confirmed_at = NULL,
                email_confirmation_code_id = NULL
            WHERE id = $1 RETURNING *",
            self.id, // $1
            email,   // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}
