use std::fmt::Display;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use cached::AsyncRedisCache;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use rust_iso3166::CountryCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::types::chrono::NaiveDate;

#[cfg(feature = "cache_remove")]
use cached::async_sync::OnceCell;
#[cfg(feature = "cache_remove")]
use cached::IOCachedAsync;

#[cfg(feature = "blob")]
mod blob;
#[cfg(feature = "confirmation-code")]
mod confirmation_code;
#[cfg(feature = "hashtag")]
mod hashtag;
#[cfg(feature = "invitation-code")]
mod invitation_code;
#[cfg(feature = "post")]
mod post;
#[cfg(feature = "post-comment")]
mod post_comment;
#[cfg(feature = "user")]
mod user;
#[cfg(feature = "user-session")]
mod user_session;
#[cfg(feature = "website")]
mod website;

#[cfg(feature = "blob")]
pub use blob::Blob;
#[cfg(feature = "confirmation-code")]
pub use confirmation_code::ConfirmationCode;
#[cfg(feature = "hashtag")]
pub use hashtag::Hashtag;
#[cfg(feature = "invitation-code")]
pub use invitation_code::InvitationCode;
#[cfg(feature = "post")]
pub use post::Post;
#[cfg(feature = "post-comment")]
pub use post_comment::PostComment;
#[cfg(feature = "user")]
pub use user::User;
#[cfg(feature = "user-session")]
pub use user_session::UserSession;
#[cfg(feature = "website")]
pub use website::Website;

use crate::config::CACHE_CONFIG;

#[cfg(feature = "cache_remove")]
pub(crate) trait AsyncRedisCacheTrait<K> {
    async fn cache_remove(&self, prefix: &str, key: &K);
}

#[cfg(feature = "cache_remove")]
impl<K, V> AsyncRedisCacheTrait<K> for OnceCell<AsyncRedisCache<K, V>>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    async fn cache_remove(&self, prefix: &str, key: &K) {
        let _ = self
            .get_or_init(|| async { async_redis_cache(prefix).await })
            .await
            .cache_remove(key)
            .await;
    }
}

pub(crate) async fn async_redis_cache<K, V>(prefix: &str) -> AsyncRedisCache<K, V>
where
    K: Display + Send + Sync,
    V: DeserializeOwned + Display + Send + Serialize + Sync,
{
    AsyncRedisCache::new(format!("{prefix}:"), CACHE_CONFIG.ttl)
        .set_connection_string(&CACHE_CONFIG.redis_url)
        .set_refresh(true)
        .build()
        .await
        .expect("Could not get redis cache")
}

fn encrypt_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

fn find_country(query: &str) -> Option<&CountryCode> {
    rust_iso3166::ALL.iter().find(|c| c.alpha2 == query || c.name == query)
}

pub fn generate_random_string(length: u8) -> String {
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
