#[cfg(not(feature = "with-dioxus"))]
use axum::extract::Host;
#[cfg(not(feature = "with-dioxus"))]
use axum_client_ip::InsecureClientIp;
#[cfg(not(feature = "with-dioxus"))]
use codee::string::FromToStringCodec;
#[cfg(feature = "with-dioxus")]
use dioxus::prelude::{extract, ServerFnError};
use http::header::{HeaderMap, ACCEPT_LANGUAGE};
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(feature = "with-dioxus")]
use unic_langid::{langid, LanguageIdentifier};

use tower_sessions::Session;

#[cfg(not(feature = "with-dioxus"))]
use mango3_core::utils::I18n;
#[cfg(not(feature = "with-dioxus"))]
use mango3_core::CoreContext;

#[cfg(not(feature = "with-dioxus"))]
use crate::context::use_language_cookie;

mod user_sessions;

#[cfg(feature = "confirmation-codes")]
mod confirmation_codes;

pub use user_sessions::*;

#[cfg(feature = "confirmation-codes")]
pub use confirmation_codes::*;

#[cfg(not(feature = "with-dioxus"))]
pub async fn extract_client_ip() -> Result<String, ServerFnError> {
    let InsecureClientIp(client_ip) = leptos_axum::extract::<InsecureClientIp>().await?;

    Ok(client_ip.to_string())
}

#[cfg(not(feature = "with-dioxus"))]
pub fn expect_core_context() -> CoreContext {
    expect_context::<CoreContext>()
}

#[cfg(not(feature = "with-dioxus"))]
pub async fn extract_host() -> Result<String, ServerFnError> {
    let Host(host) = leptos_axum::extract::<Host>().await?;

    Ok(host)
}

#[cfg(not(feature = "with-dioxus"))]
pub async fn extract_i18n() -> Result<I18n, ServerFnError> {
    let requested_language = extract_requested_language().await?;

    if let Ok(i18n) = requested_language.parse() {
        return Ok(i18n);
    }

    Ok(I18n::default())
}

#[cfg(feature = "with-dioxus")]
pub async fn extract_locale() -> Result<LanguageIdentifier, ServerFnError> {
    let session = extract_session().await?;

    if let Some(language) = session.get("language").await? {
        return Ok(language);
    }

    let header_map = extract::<HeaderMap, _>().await?;

    let accept_language = header_map.get(ACCEPT_LANGUAGE).and_then(|al| al.to_str().ok());

    let language = if let Some(accept_language) = accept_language {
        accept_language::parse(accept_language)
            .first()
            .cloned()
            .unwrap_or_else(|| "en".to_owned())
    } else {
        "en".to_owned()
    };

    language.parse().map_err(|err| ServerFnError::new(err))
}

#[cfg(not(feature = "with-dioxus"))]
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

#[cfg(feature = "with-dioxus")]
pub async fn extract_session() -> Result<Session, ServerFnError> {
    extract::<Session, _>()
        .await
        .map_err(|(_, msg)| ServerFnError::new(msg))
}

#[cfg(not(feature = "with-dioxus"))]
async fn extract_session() -> Result<Session, ServerFnError> {
    leptos_axum::extract::<Session>().await
}

#[cfg(not(feature = "with-dioxus"))]
fn try_core_context() -> Result<CoreContext, ServerFnError> {
    use_context::<CoreContext>().ok_or_else(|| ServerFnError::new("Could not find mango3_core Context"))
}
