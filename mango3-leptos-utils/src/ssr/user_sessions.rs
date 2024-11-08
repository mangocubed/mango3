use std::str::FromStr;

use codee::string::FromToStringCodec;
use leptos::prelude::*;
use uuid::Uuid;

use mango3_core::config::BASIC_CONFIG;
use mango3_core::models::{User, UserSession};
use mango3_core::CoreContext;

use crate::constants::KEY_USER_SESSION_ID;
use crate::context::use_language_cookie;
use crate::i18n::Locale;

use super::{extract_session, try_core_context};

pub async fn extract_user() -> Result<Option<User>, ServerFnError> {
    if let Some(user_session) = extract_user_session().await? {
        let core_context = try_core_context()?;

        Ok(user_session.user(&core_context).await.ok())
    } else {
        Ok(None)
    }
}

pub async fn extract_user_session() -> Result<Option<UserSession>, ServerFnError> {
    let session = extract_session().await?;

    let Some(id) = session.get::<Uuid>(KEY_USER_SESSION_ID).await? else {
        return Ok(None);
    };

    let core_context = try_core_context()?;

    Ok(UserSession::get_by_id(&core_context, id).await.ok())
}

pub async fn finish_user_session(core_context: &CoreContext) -> Result<(), ServerFnError> {
    let Some(user_session) = extract_user_session().await? else {
        return Ok(());
    };

    let (_, set_cookie_lang) = use_language_cookie::<FromToStringCodec>();

    set_cookie_lang.set(None);

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
    let (_, set_cookie_lang) = use_language_cookie::<FromToStringCodec>();

    let user_session = UserSession::insert(&core_context, user)
        .await
        .map_err(|_| ServerFnError::new("Could not start user session.".to_owned()))?;

    session.insert(KEY_USER_SESSION_ID, user_session.id).await?;

    set_cookie_lang.set(Locale::from_str(&user.language_code).ok());

    Ok(user_session)
}
