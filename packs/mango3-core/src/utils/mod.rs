mod info;

#[cfg(feature = "cache")]
mod cache_utils;
#[cfg(feature = "handlebars")]
mod handlebars_utils;
#[cfg(feature = "jobs")]
mod jobs;
#[cfg(feature = "locales")]
mod locales;
#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "mutation")]
mod mutation;
#[cfg(feature = "pagination")]
mod pagination;
#[cfg(feature = "text-icon")]
mod text_icon;
#[cfg(feature = "validator")]
mod validator;

pub use info::{Info, INFO};

#[cfg(feature = "cache")]
pub(crate) use cache_utils::{async_redis_cache, AsyncRedisCacheTrait};
#[cfg(feature = "handlebars")]
pub use handlebars_utils::render_handlebars;
#[cfg(feature = "jobs")]
pub use jobs::{AdminMailerJob, GuestMailerJob, Jobs, MailerJob};
#[cfg(feature = "locales")]
pub use locales::I18n;
#[cfg(feature = "markdown")]
pub use markdown::parse_html;
#[cfg(feature = "mutation")]
pub use mutation::{MutError, MutResult, MutSuccess};
#[cfg(feature = "pagination")]
pub use pagination::{cursor_page, CursorPage, CursorPageParams};
#[cfg(feature = "text-icon")]
pub use text_icon::text_icon;
#[cfg(feature = "validator")]
pub use validator::{ValidationErrors, Validator, ValidatorTrait};

#[cfg(feature = "encrypt-password")]
pub fn encrypt_password(password: &str) -> String {
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

#[cfg(feature = "generate-random-string")]
pub(crate) fn generate_random_string(length: u8) -> String {
    use rand::distr::Alphanumeric;
    use rand::{rng, Rng};

    rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

#[cfg(feature = "find-country")]
pub fn find_country(query: &str) -> Option<&rust_iso3166::CountryCode> {
    rust_iso3166::ALL.iter().find(|c| c.alpha2 == query || c.name == query)
}

#[cfg(feature = "parse-date")]
pub fn parse_date(value: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

#[cfg(feature = "verify-password")]
pub(crate) fn verify_password(password: &str, encrypted_password: &str) -> bool {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(encrypted_password);

    if password_hash.is_err() {
        return false;
    }

    let password_hash = password_hash.unwrap();

    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}
