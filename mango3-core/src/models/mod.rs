use std::fmt::Display;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use cached::async_sync::OnceCell;
use cached::{AsyncRedisCache, IOCachedAsync};
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use rust_iso3166::CountryCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::types::chrono::NaiveDate;

mod blob;
mod confirmation_code;
mod hashtag;
mod invitation_code;
mod navigation_item;
mod post;
mod post_comment;
mod post_reaction;
mod post_view;
mod user;
mod user_password_reset;
mod user_session;
mod website;

pub use blob::Blob;
pub use confirmation_code::ConfirmationCode;
pub use hashtag::Hashtag;
pub use invitation_code::InvitationCode;
pub use navigation_item::NavigationItem;
pub use post::Post;
pub use post_comment::PostComment;
pub use post_reaction::PostReaction;
pub use post_view::PostView;
pub use user::User;
pub use user_password_reset::UserPasswordReset;
pub use user_session::UserSession;
pub use website::Website;

trait AsyncRedisCacheTrait<K> {
    async fn cache_remove(&self, key: &K);
}

impl<K, V> AsyncRedisCacheTrait<K> for OnceCell<AsyncRedisCache<K, V>>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Send + Serialize + Sync,
{
    async fn cache_remove(&self, key: &K) {
        if let Some(cache) = self.get() {
            let _ = cache.cache_remove(key).await;
        }
    }
}

fn encrypt_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

fn find_country(query: &str) -> Option<&CountryCode> {
    rust_iso3166::ALL.iter().find(|c| c.alpha2 == query || c.name == query)
}

fn generate_random_string(length: u8) -> String {
    rng()
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
