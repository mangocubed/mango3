use sqlx::query_as;

use crate::constants::KEY_TEXT_RESET_YOUR_PASSWORD;
use crate::enums::{Input, InputError};
use crate::models::{encrypt_password, verify_password, ConfirmationCode};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn password_reset_confirmation_code(
        &self,
        core_context: &CoreContext,
    ) -> Option<sqlx::Result<ConfirmationCode>> {
        if let Some(password_reset_confirmation_code_id) = self.password_reset_confirmation_code_id {
            Some(ConfirmationCode::get_by_id(core_context, password_reset_confirmation_code_id).await)
        } else {
            None
        }
    }
    pub async fn send_password_reset_confirmation_code(
        &self,
        core_context: &CoreContext,
    ) -> Result<Self, ValidationErrors> {
        let i18n = self.i18n();
        let action = i18n.text(KEY_TEXT_RESET_YOUR_PASSWORD);

        let confirmation_code = ConfirmationCode::insert(core_context, self, &action).await?;

        query_as!(
            Self,
            "UPDATE users SET password_reset_confirmation_code_id = $2 WHERE id = $1 RETURNING *",
            self.id,              // $1
            confirmation_code.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn update_password(
        &self,
        core_context: &CoreContext,
        current_password: &str,
        new_password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        if validator.validate_presence(Input::CurrentPassword, current_password) {
            validator.custom_validation(Input::CurrentPassword, InputError::IsInvalid, &|| {
                self.verify_password(current_password)
            });
        }

        validator.validate_password(Input::NewPassword, new_password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        if self.verify_password(new_password) {
            return Ok(self.clone());
        }

        let encrypted_password = encrypt_password(new_password);

        query_as!(
            Self,
            "UPDATE users SET encrypted_password = $2 WHERE id = $1 RETURNING *",
            self.id,            // $1
            encrypted_password, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn update_password_with_code(
        &self,
        core_context: &CoreContext,
        code: &str,
        new_password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        if validator.validate_presence(Input::Code, code) {
            let password_reset_confirmation_code = self
                .password_reset_confirmation_code(core_context)
                .await
                .ok_or_else(ValidationErrors::default)?
                .map_err(|_| ValidationErrors::default())?;
            let code_is_verified = password_reset_confirmation_code.verify_code(core_context, code).await;

            validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
        }

        validator.validate_password(Input::NewPassword, new_password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let encrypted_password = encrypt_password(new_password);

        query_as!(
            Self,
            "UPDATE users SET encrypted_password = $2, password_reset_confirmation_code_id = NULL WHERE id = $1
            RETURNING *",
            self.id,            // $1
            encrypted_password, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        if self.encrypted_password.is_empty() {
            return false;
        }

        verify_password(password, &self.encrypted_password)
    }
}
