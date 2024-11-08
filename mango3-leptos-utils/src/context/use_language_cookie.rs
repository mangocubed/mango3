use codee::{Decoder, Encoder};
use leptos::prelude::*;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};

use crate::constants::COOKIE_NAME_LANGUAGE;
use crate::i18n::Locale;

use super::use_basic_config;

pub fn use_language_cookie<C>() -> (Signal<Option<Locale>>, WriteSignal<Option<Locale>>)
where
    C: Decoder<Locale, Encoded = str> + Encoder<Locale, Encoded = String>,
{
    use_cookie_with_options::<Locale, C>(COOKIE_NAME_LANGUAGE, use_language_cookie_options::<C>())
}

pub fn use_language_cookie_options<C>(
) -> UseCookieOptions<Locale, <C as Encoder<Locale>>::Error, <C as Decoder<Locale>>::Error>
where
    C: Encoder<Locale, Encoded = String> + Decoder<Locale, Encoded = str>,
{
    let basic_config = use_basic_config();

    UseCookieOptions::default()
        .domain(basic_config.domain.clone())
        .same_site(SameSite::Strict)
}
