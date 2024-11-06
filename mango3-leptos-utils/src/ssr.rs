use axum_extra::extract::cookie::CookieJar;
use http::header::{HeaderMap, ACCEPT_LANGUAGE};
use leptos::prelude::*;
use tower_sessions::Session;
use uuid::Uuid;

use mango3_core::config::BASIC_CONFIG;
use mango3_core::locales::I18n;
use mango3_core::models::{User, UserSession};
use mango3_core::CoreContext;

use crate::constants::{COOKIE_NAME_LANGUAGE, KEY_USER_SESSION_ID};

pub fn expect_core_context() -> CoreContext {
    expect_context::<CoreContext>()
}

pub async fn extract_i18n() -> Result<I18n, ServerFnError> {
    let requested_language = extract_requested_language().await?;

    if let Ok(i18n) = requested_language.parse() {
        return Ok(i18n);
    }

    Ok(I18n::default())
}

async fn extract_requested_language() -> Result<String, ServerFnError> {
    let cookie_jar = leptos_axum::extract::<CookieJar>().await?;
    let cookie = cookie_jar.get(COOKIE_NAME_LANGUAGE);

    if let Some(cookie) = cookie {
        return Ok(cookie.value().to_owned());
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

pub async fn extract_user() -> Result<Option<User>, ServerFnError> {
    if let Some(user_session) = extract_user_session().await? {
        let core_context = expect_core_context();

        Ok(user_session.user(&core_context).await.ok())
    } else {
        Ok(None)
    }
}

pub async fn extract_user_session() -> Result<Option<UserSession>, ServerFnError> {
    let core_context = expect_core_context();
    let session = extract_session().await?;

    let Some(id) = session.get::<Uuid>(KEY_USER_SESSION_ID).await? else {
        return Ok(None);
    };

    Ok(UserSession::get_by_id(&core_context, id).await.ok())
}

pub async fn finish_user_session(core_context: &CoreContext) -> Result<(), ServerFnError> {
    let Some(user_session) = extract_user_session().await? else {
        return Ok(());
    };

    user_session
        .delete(&core_context)
        .await
        .map_err(|_| ServerFnError::new("Could not finish user session.".to_owned()))?;

    let session = extract_session().await?;

    session.remove::<Uuid>(KEY_USER_SESSION_ID).await?;

    Ok(())
}

pub(crate) async fn is_authenticated() -> Result<bool, ServerFnError> {
    Ok(crate::ssr::extract_user_session().await?.is_some())
}

pub async fn require_authentication() -> Result<bool, ServerFnError> {
    if !is_authenticated().await? {
        leptos_axum::redirect(BASIC_CONFIG.login_url().as_str());

        return Ok(false);
    }

    Ok(true)
}

pub async fn require_no_authentication() -> Result<bool, ServerFnError> {
    if is_authenticated().await? {
        leptos_axum::redirect(BASIC_CONFIG.home_url().as_str());

        return Ok(false);
    }

    Ok(true)
}

pub async fn start_user_session(core_context: &CoreContext, user: &User) -> Result<UserSession, ServerFnError> {
    let session = extract_session().await?;
    let user_session = UserSession::insert(&core_context, user)
        .await
        .map_err(|_| ServerFnError::new("Could not start user session.".to_owned()))?;

    session.insert(KEY_USER_SESSION_ID, user_session.id).await?;

    Ok(user_session)
}
