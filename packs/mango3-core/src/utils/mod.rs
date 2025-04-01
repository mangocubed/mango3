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
