use sqlx::{query, query_as};

use crate::constants::REGEX_EMAIL;
use crate::enums::{ConfirmationCodeAction, Input, InputError, UserRole};
use crate::models::ConfirmationCode;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn confirm_email(&self, core_context: &CoreContext) -> Result<Self, ValidationErrors> {
        let result = query_as!(
            Self,
            r#"UPDATE users SET email_confirmed_at = current_timestamp
            WHERE disabled_at IS NULL AND email_confirmed_at IS NULL AND id = $1 RETURNING
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
            self.id, // $1
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

    pub fn email_is_confirmed(&self) -> bool {
        self.email_confirmed_at.is_some()
    }

    pub async fn send_email_confirmation_code(
        &self,
        core_context: &CoreContext,
    ) -> Result<ConfirmationCode, ValidationErrors> {
        if self.email_is_confirmed() {
            return Err(ValidationErrors::default());
        }

        ConfirmationCode::insert(core_context, self, ConfirmationCodeAction::EmailConfirmation)
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
            && validator.validate_length(Input::Email, &email, Some(5), Some(256))
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

        let result = query_as!(
            Self,
            r#"UPDATE users SET email = $2::text, email_confirmed_at = NULL WHERE disabled_at IS NULL AND id = $1
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
            self.id, // $1
            email,   // $2
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
}
