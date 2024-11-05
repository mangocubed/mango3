use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use rust_iso3166::CountryCode;
use sqlx::types::chrono::NaiveDate;

mod user;

pub use user::User;

fn encrypt_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

fn find_country(query: &str) -> Option<&CountryCode> {
    rust_iso3166::ALL.iter().find(|c| c.alpha2 == query || c.name == query)
}

fn parse_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}
