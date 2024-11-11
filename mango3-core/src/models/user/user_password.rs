use sqlx::query_as;

use crate::enums::{Input, InputError};
use crate::models::{encrypt_password, verify_password};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
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

    pub fn verify_password(&self, password: &str) -> bool {
        if self.encrypted_password.is_empty() {
            return false;
        }

        verify_password(password, &self.encrypted_password)
    }
}
