use rust_iso3166::CountryCode;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::Uuid;

use crate::enums::{Input, InputError};
use crate::locales::I18n;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

mod user_insert;
mod user_password;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_deserializing, skip_serializing)]
    encrypted_password: String,
    pub display_name: String,
    pub full_name: String,
    pub birthdate: NaiveDate,
    pub language_code: String,
    pub country_alpha2: String,
    pub bio: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn authenticate(
        core_context: &CoreContext,
        username_or_email: &str,
        password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        validator.validate_presence(Input::UsernameOrEmail, username_or_email);
        validator.validate_presence(Input::Password, password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let user = Self::get_by_username_or_email(core_context, username_or_email)
            .await
            .map_err(|_| ValidationErrors::default())?;

        if user.verify_password(password) {
            Ok(user)
        } else {
            Err(ValidationErrors::default())
        }
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Self> {
        query_as!(Self, "SELECT * FROM users WHERE id = $1 LIMIT 1", id)
            .fetch_one(&core_context.db_pool)
            .await
    }

    pub async fn get_by_username_or_email(core_context: &CoreContext, username_or_email: &str) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "SELECT * FROM users WHERE LOWER(username) = $1 OR LOWER(email) = $1 LIMIT 1",
            username_or_email.to_lowercase()
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub fn i18n(&self) -> I18n {
        I18n::from(self.language_code.as_str())
    }

    pub fn initials(&self) -> String {
        self.display_name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase()
    }
}

impl Validator {
    fn validate_full_name(&mut self, value: &str) -> bool {
        if self.validate_presence(Input::FullName, value) {
            return self.validate_length(Input::FullName, value, Some(2), Some(256));
        }

        false
    }

    fn validate_birthdate(&mut self, value: Option<NaiveDate>) -> bool {
        self.validate_presence(Input::Birthdate, value)
            && self.custom_validation(Input::Birthdate, InputError::IsInvalid, &|| {
                value.unwrap() <= Utc::now().date_naive()
            })
    }

    fn validate_country(&mut self, value: Option<&CountryCode>) -> bool {
        self.validate_presence(Input::CountryAlpha2, value)
    }
}
