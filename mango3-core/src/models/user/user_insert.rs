use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::config::USER_CONFIG;
use crate::constants::{BLACKLISTED_SLUGS, REGEX_EMAIL, REGEX_USERNAME};
use crate::enums::{Input, InputError, MailerJobCommand, UserRole};
use crate::models::{encrypt_password, find_country, parse_date};
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn insert(
        core_context: &CoreContext,
        username: &str,
        email: &str,
        password: &str,
        full_name: &str,
        birthdate: &str,
        language_code: &str,
        country_alpha2: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let username = username.trim();
        let email = email.trim().to_lowercase();
        let full_name = full_name.trim();
        let birthdate = parse_date(birthdate);
        let country = find_country(country_alpha2);

        if validator.validate_presence(Input::Username, username)
            && validator.validate_length(Input::Username, username, Some(3), Some(16))
            && validator.validate_format(Input::Username, username, &REGEX_USERNAME)
            && validator.custom_validation(Input::Username, InputError::IsInvalid, &|| {
                Uuid::try_parse(username).is_err()
            })
            && validator.custom_validation(Input::Username, InputError::IsInvalid, &|| {
                !BLACKLISTED_SLUGS.contains(&username.to_lowercase().as_str())
            })
        {
            let username_exists = query!(
                "SELECT id FROM users WHERE LOWER(username) = $1 LIMIT 1",
                username.to_lowercase() // $1
            )
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok();
            validator.custom_validation(Input::Username, InputError::AlreadyInUse, &|| !username_exists);
        }

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

        validator.validate_password(Input::Password, password);

        validator.validate_full_name(full_name);

        validator.validate_birthdate(birthdate);

        validator.validate_country(country);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let display_name = full_name.split(' ').next().unwrap();
        let encrypted_password = encrypt_password(password);

        let result = query_as!(
            Self,
            r#"INSERT INTO users (
                username,
                email,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                role,
                disabled_at
            ) VALUES (
                $1::text, $2::text, $3, $4, $5, $6, $7, $8, $9,
                (CASE WHEN $10 IS TRUE THEN current_timestamp ELSE NULL END)
            )
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
                disabled_at,
                created_at,
                updated_at"#,
            username,                                 // $1
            email,                                    // $2
            encrypted_password,                       // $3
            display_name,                             // $4
            full_name,                                // $5
            birthdate,                                // $6
            language_code,                            // $7
            country.unwrap().alpha2,                  // $8
            &USER_CONFIG.default_role() as &UserRole, // $9
            USER_CONFIG.default_disabled,             // $10
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                core_context.jobs.mailer(&user, MailerJobCommand::Welcome).await;

                Ok(user)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
