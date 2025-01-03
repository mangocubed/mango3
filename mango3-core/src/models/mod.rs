use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rust_iso3166::CountryCode;
use sqlx::types::chrono::NaiveDate;

mod blob;
mod confirmation_code;
mod invitation_code;
mod navigation_item;
mod post;
mod post_attachment;
mod post_view;
mod user;
mod user_session;
mod website;

pub use blob::Blob;
pub use confirmation_code::ConfirmationCode;
pub use invitation_code::InvitationCode;
pub use navigation_item::NavigationItem;
pub use post::Post;
pub use post_attachment::PostAttachment;
pub use post_view::PostView;
pub use user::User;
pub use user_session::UserSession;
pub use website::Website;

fn encrypt_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

fn find_country(query: &str) -> Option<&CountryCode> {
    rust_iso3166::ALL.iter().find(|c| c.alpha2 == query || c.name == query)
}

fn generate_random_string(length: u8) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

fn parse_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()
}

fn verify_password(password: &str, encrypted_password: &str) -> bool {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(encrypted_password);

    if password_hash.is_err() {
        return false;
    }

    let password_hash = password_hash.unwrap();

    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}
