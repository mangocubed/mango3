use axum::extract::Host;
use codee::string::FromToStringCodec;
use http::header::{HeaderMap, ACCEPT_LANGUAGE};
use leptos::prelude::*;
use tower_sessions::Session;

use mango3_core::locales::I18n;
use mango3_core::CoreContext;

use crate::context::use_language_cookie;

mod user_sessions;

pub use user_sessions::*;

pub fn expect_core_context() -> CoreContext {
    expect_context::<CoreContext>()
}

pub async fn extract_host() -> Result<String, ServerFnError> {
    let Host(host) = leptos_axum::extract::<Host>().await?;

    Ok(host)
}

pub async fn extract_i18n() -> Result<I18n, ServerFnError> {
    let requested_language = extract_requested_language().await?;

    if let Ok(i18n) = requested_language.parse() {
        return Ok(i18n);
    }

    Ok(I18n::default())
}

async fn extract_requested_language() -> Result<String, ServerFnError> {
    let (cookie, _) = use_language_cookie::<FromToStringCodec>();

    if let Some(cookie) = cookie.get() {
        return Ok(cookie.to_string());
    }

    let header_map = leptos_axum::extract::<HeaderMap>().await?;
    let accept_language = header_map.get(ACCEPT_LANGUAGE).and_then(|al| al.to_str().ok());

    if let Some(accept_language) = accept_language {
        let primary_language = accept_language::parse(accept_language)
            .first()
            .cloned()
            .unwrap_or_else(|| "en".to_owned());

        return Ok(primary_language);
    }

    Ok("en".to_owned())
}

async fn extract_session() -> Result<Session, ServerFnError> {
    leptos_axum::extract::<Session>().await
}

fn try_core_context() -> Result<CoreContext, ServerFnError> {
    use_context::<CoreContext>().ok_or_else(|| ServerFnError::new("Could not find Core Context"))
}
