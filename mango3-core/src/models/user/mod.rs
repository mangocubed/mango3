use cached::IOCachedAsync;
use rust_iso3166::CountryCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::BASIC_CONFIG;
use crate::enums::{Input, InputError, UserRole};
use crate::locales::I18n;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{Blob, Hashtag, Website};

mod user_bio;
mod user_email;
mod user_get;
mod user_insert;
mod user_lock;
mod user_password;
mod user_profile;

use user_bio::USER_BIO_HTML;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    email_confirmation_code_id: Option<Uuid>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    #[serde(skip_deserializing, skip_serializing)]
    encrypted_password: String,
    password_reset_confirmation_code_id: Option<Uuid>,
    pub display_name: String,
    pub full_name: String,
    pub birthdate: NaiveDate,
    pub language_code: String,
    pub country_alpha2: String,
    pub bio: String,
    pub hashtag_ids: Vec<Uuid>,
    pub avatar_image_blob_id: Option<Uuid>,
    pub role: UserRole,
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

    pub async fn avatar_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.avatar_image_blob_id {
            Some(Blob::get_by_id(core_context, id, Some(self)).await)
        } else {
            None
        }
    }

    fn cache_remove(&self) {
        USER_BIO_HTML.get().map(|cache| cache.cache_remove(&self.id));
    }

    pub async fn can_insert_website(&self, core_context: &CoreContext) -> bool {
        self.role != UserRole::User
            || Website::count(core_context, Some(self))
                .await
                .expect("could not get websites count")
                < 1
    }

    pub fn country(&self) -> CountryCode {
        rust_iso3166::from_alpha2(&self.country_alpha2).unwrap()
    }

    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        Hashtag::all_by_ids(core_context, &self.hashtag_ids).await
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

    pub fn text_avatar_url(&self) -> Url {
        BASIC_CONFIG.text_icon_url(&self.initials())
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.user_url(&self.username)
    }
}

impl Validator {
    fn validate_full_name(&mut self, value: &str) -> bool {
        self.validate_presence(Input::FullName, value)
            && self.validate_length(Input::FullName, value, Some(2), Some(256))
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

    fn validate_password(&mut self, input: Input, value: &str) -> bool {
        self.validate_presence(input.clone(), value) && self.validate_length(input, value, Some(6), Some(128))
    }
}
