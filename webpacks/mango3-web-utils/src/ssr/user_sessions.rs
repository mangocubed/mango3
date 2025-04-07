use leptos::prelude::*;
use uuid::Uuid;

use mango3_core::commands::get_user_session_by_id;
use mango3_core::config::BASIC_CONFIG;
use mango3_core::models::{User, UserSession};

use crate::constants::KEY_USER_SESSION_ID;

pub async fn extract_user() -> Result<Option<User>, ServerFnError> {
    if let Some(user_session) = extract_user_session().await? {
        let core_context = super::try_core_context()?;

        Ok(user_session.user(&core_context).await.ok())
    } else {
        Ok(None)
    }
}

pub async fn extract_user_session() -> Result<Option<UserSession>, ServerFnError> {
    let session = super::extract_session().await?;

    let Some(id) = session.get::<Uuid>(KEY_USER_SESSION_ID).await? else {
        return Ok(None);
    };

    let core_context = super::try_core_context()?;

    Ok(get_user_session_by_id(&core_context, id).await.ok())
}

#[cfg(feature = "finish-and-delete-user-session")]
pub async fn finish_and_delete_user_session(core_context: &mango3_core::CoreContext) -> Result<(), ServerFnError> {
    let Some(user_session) = extract_user_session().await? else {
        return Ok(());
    };

    let (_, set_cookie_lang) = crate::context::use_language_cookie::<codee::string::FromToStringCodec>();

    set_cookie_lang.set(None);

    mango3_core::commands::delete_user_session(&core_context, &user_session)
        .await
        .map_err(|_| ServerFnError::new("Could not delete user session.".to_owned()))?;

    let session = super::extract_session().await?;

    session.remove::<Uuid>(KEY_USER_SESSION_ID).await?;

    Ok(())
}

pub(crate) async fn is_authenticated() -> Result<bool, ServerFnError> {
    Ok(extract_user_session().await?.is_some())
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

#[cfg(feature = "start-user-session")]
pub async fn start_user_session(
    core_context: &mango3_core::CoreContext,
    user_session: &UserSession,
) -> Result<(), ServerFnError> {
    use std::str::FromStr;

    let user = user_session.user(core_context).await?;
    let session = super::extract_session().await?;
    let (_, set_cookie_lang) = crate::context::use_language_cookie::<codee::string::FromToStringCodec>();

    session.insert(KEY_USER_SESSION_ID, user_session.id).await?;

    set_cookie_lang.set(crate::i18n::Locale::from_str(&user.language_code).ok());

    Ok(())
}
