#[cfg(feature = "user_session_insert")]
use std::str::FromStr;

use leptos::prelude::*;
use uuid::Uuid;

#[cfg(any(feature = "user_session_delete", feature = "user_session_insert"))]
use codee::string::FromToStringCodec;

use mango3_core::commands::UserSessionGet;
use mango3_core::config::BASIC_CONFIG;
use mango3_core::models::User;
use mango3_utils::models::UserSession;

#[cfg(feature = "user_session_delete")]
use mango3_core::commands::UserSessionDelete;
#[cfg(any(feature = "user_session_delete", feature = "user_session_insert"))]
use mango3_core::CoreContext;

use crate::constants::KEY_USER_SESSION_ID;

#[cfg(any(feature = "user_session_delete", feature = "user_session_insert"))]
use crate::context::use_language_cookie;
#[cfg(feature = "user_session_insert")]
use crate::i18n::Locale;

use super::{extract_session, try_core_context};

#[cfg(feature = "extract_user")]
pub async fn extract_user() -> Result<Option<User>, ServerFnError> {
    if let Some(user_session) = extract_user_session().await? {
        let core_context = try_core_context()?;

        Ok(
            mango3_core::commands::get_user_by_id(&core_context, user_session.user_id)
                .await
                .ok(),
        )
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

#[cfg(feature = "user_session_delete")]
pub async fn finish_and_delete_user_session(core_context: &CoreContext) -> Result<(), ServerFnError> {
    let Some(user_session) = extract_user_session().await? else {
        return Ok(());
    };

    let (_, set_cookie_lang) = use_language_cookie::<FromToStringCodec>();

    set_cookie_lang.set(None);

    user_session
        .delete(&core_context)
        .await
        .map_err(|_| ServerFnError::new("Could not delete user session.".to_owned()))?;

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

#[cfg(feature = "user_session_insert")]
pub async fn start_user_session(core_context: &CoreContext, user_session: &UserSession) -> Result<(), ServerFnError> {
    let user = User::get_by_id(core_context, user_session.user_id).await?;

    let session = extract_session().await?;
    let (_, set_cookie_lang) = use_language_cookie::<FromToStringCodec>();

    session.insert(KEY_USER_SESSION_ID, user_session.id).await?;

    set_cookie_lang.set(Locale::from_str(&user.language_code).ok());

    Ok(())
}
