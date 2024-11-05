use axum_extra::extract::cookie::CookieJar;
use http::header::{HeaderMap, ACCEPT_LANGUAGE};
use leptos::prelude::*;

use mango3_core::locales::I18n;
use mango3_core::CoreContext;

use crate::constants::COOKIE_NAME_LANGUAGE;

pub fn expect_core_context() -> CoreContext {
    expect_context::<CoreContext>()
}

pub async fn extract_i18n() -> Result<I18n, ServerFnError> {
    let cookie_jar = leptos_axum::extract::<CookieJar>().await?;
    let cookie = cookie_jar.get(COOKIE_NAME_LANGUAGE);

    if let Some(cookie) = cookie {
        if let Ok(i18n) = cookie.value().parse() {
            return Ok(i18n);
        }
    }

    let header_map = leptos_axum::extract::<HeaderMap>().await?;
    let accept_language = header_map.get(ACCEPT_LANGUAGE).and_then(|al| al.to_str().ok());

    if let Some(accept_language) = accept_language {
        let primary_language = accept_language::parse(accept_language)
            .first()
            .cloned()
            .unwrap_or_else(|| "en".to_owned());

        if let Ok(i18n) = primary_language.parse() {
            return Ok(i18n);
        }
    }

    Ok(I18n::default())
}
