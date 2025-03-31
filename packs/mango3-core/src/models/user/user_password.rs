#[cfg(feature = "user_cache_remove")]
use sqlx::query_as;

use crate::enums::ConfirmationCodeAction;
use crate::models::{verify_password, ConfirmationCode};
use crate::validator::ValidationErrors;
use crate::CoreContext;

#[cfg(feature = "user_cache_remove")]
use crate::enums::{Input, InputError, UserRole};
#[cfg(feature = "user_cache_remove")]
use crate::models::encrypt_password;
#[cfg(feature = "user_cache_remove")]
use crate::validator::{Validator, ValidatorTrait};

use super::User;

impl User {
    #[cfg(feature = "user_cache_remove")]
    pub async fn reset_password(
        &self,
        core_context: &CoreContext,
        new_password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        validator.validate_password(Input::NewPassword, new_password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let encrypted_password = encrypt_password(new_password);

        let result = query_as!(
            Self,
            r#"UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1
                RETURNING
                    id,
                    username,
                    email,
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
                    disabled_at,
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

    pub async fn send_password_reset_code(
        &self,
        core_context: &CoreContext,
    ) -> Result<ConfirmationCode, ValidationErrors> {
        if !self.email_is_confirmed() {
            return Err(ValidationErrors::default());
        }

        ConfirmationCode::insert(core_context, self, ConfirmationCodeAction::PasswordReset)
            .await
            .map_err(|_| ValidationErrors::default())
    }

    #[cfg(feature = "user_cache_remove")]
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
            r#"UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1 RETURNING
                id,
                username,
                email,
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
                disabled_at,
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

    pub fn verify_password(&self, password: &str) -> bool {
        if self.encrypted_password.is_empty() {
            return false;
        }

        verify_password(password, &self.encrypted_password)
    }
}
