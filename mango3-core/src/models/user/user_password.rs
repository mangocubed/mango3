use sqlx::query_as;

use crate::enums::{Input, InputError, UserRole};
use crate::models::user_password_reset::UserPasswordReset;
use crate::models::{encrypt_password, verify_password, ConfirmationCode};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn password_reset(&self, core_context: &CoreContext) -> sqlx::Result<UserPasswordReset> {
        UserPasswordReset::get_by_user(core_context, self).await
    }

    pub async fn password_reset_confirmation_code(&self, core_context: &CoreContext) -> sqlx::Result<ConfirmationCode> {
        self.password_reset(core_context)
            .await?
            .confirmation_code(core_context)
            .await
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

        let result = query_as!(
            Self,
            r#"UPDATE users SET encrypted_password = $2 WHERE locked_at IS NULL AND id = $1 RETURNING
                id,
                username,
                email,
                email_confirmation_code_id,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                created_at,
                updated_at"#,
            self.id,            // $1
            encrypted_password, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                user.cache_remove().await;

                Ok(user)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn update_password_with_code(
        &self,
        core_context: &CoreContext,
        code: &str,
        new_password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let confirmation_code = self
            .password_reset_confirmation_code(core_context)
            .await
            .map_err(|_| ValidationErrors::default())?;

        if validator.validate_presence(Input::Code, code) {
            let code_is_verified = confirmation_code.verify_code(core_context, code).await;

            validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
        }

        validator.validate_password(Input::NewPassword, new_password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let encrypted_password = encrypt_password(new_password);

        let result = query_as!(
            Self,
            r#"UPDATE users SET encrypted_password = $2 WHERE locked_at IS NULL AND id = $1
            RETURNING
                id,
                username,
                email,
                email_confirmation_code_id,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                created_at,
                updated_at"#,
            self.id,            // $1
            encrypted_password, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                let _ = confirmation_code.delete(core_context).await;

                user.cache_remove().await;

                Ok(user)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        if self.encrypted_password.is_empty() {
            return false;
        }

        verify_password(password, &self.encrypted_password)
    }
}
