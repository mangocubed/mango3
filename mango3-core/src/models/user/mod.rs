use rust_iso3166::CountryCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::Uuid;

use crate::enums::{Input, InputError};
use crate::locales::I18n;
use crate::validator::{Validator, ValidatorTrait};

mod user_insert;

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
    pub fn i18n(&self) -> I18n {
        I18n::from(self.language_code.as_str())
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
