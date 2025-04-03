use std::fmt::Display;

use rust_iso3166::CountryCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::BASIC_CONFIG;
use crate::enums::UserRole;
use crate::CoreContext;

use super::{Blob, Hashtag, Website};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub(crate) encrypted_password: String,
    pub display_name: String,
    pub full_name: String,
    pub birthdate: NaiveDate,
    pub language_code: String,
    pub country_alpha2: String,
    pub bio: String,
    pub hashtag_ids: Vec<Uuid>,
    pub avatar_image_blob_id: Option<Uuid>,
    pub role: UserRole,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl User {
    pub async fn avatar_image_blob(&self, core_context: &CoreContext) -> Option<sqlx::Result<Blob>> {
        if let Some(id) = self.avatar_image_blob_id {
            Some(crate::commands::get_blob_by_id(core_context, id, None, Some(self)).await)
        } else {
            None
        }
    }

    #[cfg(feature = "user-bio-html")]
    pub async fn bio_html(&self) -> String {
        user_bio_html(self).await.unwrap_or_default()
    }

    #[cfg(feature = "user-bio-preview-html")]
    pub async fn bio_preview_html(&self) -> String {
        user_bio_preview_html(self).await.unwrap_or_default()
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

    #[cfg(feature = "user-email-is-confirmed")]
    pub fn email_is_confirmed(&self) -> bool {
        self.email_confirmed_at.is_some()
    }

    pub async fn hashtags(&self, core_context: &CoreContext) -> Vec<Hashtag> {
        crate::commands::all_hashtags_by_ids(core_context, &self.hashtag_ids).await
    }

    #[cfg(feature = "user-i18n")]
    pub fn i18n(&self) -> crate::utils::I18n {
        crate::utils::I18n::from(self.language_code.as_str())
    }

    pub fn initials(&self) -> String {
        self.display_name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled_at.is_some()
    }

    pub fn text_avatar_url(&self) -> Url {
        BASIC_CONFIG.text_icon_url(&self.initials())
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.user_url(&self.username)
    }
}

#[cfg(feature = "user-bio-html")]
#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ user.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_USER_BIO_HTML).await } "##
)]
pub(crate) async fn user_bio_html(user: &User) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(&user.bio, true))
}

#[cfg(feature = "user-bio-preview-html")]
#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ user.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_USER_BIO_PREVIEW_HTML).await } "##
)]
pub(crate) async fn user_bio_preview_html(user: &User) -> Result<String, cached::RedisCacheError> {
    Ok(crate::parse_html!(
        &user
            .bio
            .lines()
            .next()
            .map(|line| line.get(..256).unwrap_or(line).trim().to_owned())
            .unwrap_or_default(),
        false
    ))
}
