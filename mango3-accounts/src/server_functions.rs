use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::ActionFormResp;

#[cfg(feature = "ssr")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "ssr")]
use mango3_core::models::{InvitationCode, User, UserPasswordReset, UserSession};
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{
    expect_core_context, extract_confirmation_code, extract_i18n, finish_confirmation_code, require_no_authentication,
    start_confirmation_code, start_user_session,
};

use crate::models::UserSessionResp;

#[server]
pub async fn attempt_to_confirm_login(code: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let user_session = UserSession::get_by_confirmation_code(&core_context, &confirmation_code).await?;

    let result = user_session.confirm(&core_context, &code).await;

    match result {
        Ok(ref user_session) => {
            let _ = start_user_session(&core_context, &user_session).await;
            let _ = finish_confirmation_code().await;

            ActionFormResp::new(&i18n, result)
        }
        _ => ActionFormResp::new_with_error(&i18n),
    }
}

#[server]
pub async fn attempt_to_login(
    username_or_email: String,
    password: String,
) -> Result<ActionFormResp<UserSessionResp>, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let Ok(user) = User::authenticate(&core_context, &username_or_email, &password).await else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let result = UserSession::insert(&core_context, &user, false).await;

    match result {
        Ok(ref user_session) => {
            let user_session_resp = UserSessionResp::from(user_session);

            if let Some(Ok(confirmation_code)) = user_session.confirmation_code(&core_context).await {
                let _ = start_confirmation_code(&confirmation_code).await;
            } else {
                let _ = start_user_session(&core_context, &user_session).await;
            }

            ActionFormResp::new_with_data(&i18n, result, user_session_resp)
        }
        _ => ActionFormResp::new_with_error(&i18n),
    }
}

#[server]
pub async fn attempt_to_register(
    invitation_code_id: Option<String>,
    username: String,
    email: String,
    password: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let invitation_code = if !BASIC_CONFIG.enable_register {
        let Some(id) = invitation_code_id else {
            return ActionFormResp::new_with_error(&i18n);
        };

        Some(InvitationCode::get_by_id(&core_context, Uuid::try_parse(&id)?).await?)
    } else {
        None
    };

    let result = User::insert(
        &core_context,
        &username,
        &email,
        &password,
        &full_name,
        &birthdate,
        i18n.0.language.as_str(),
        &country_alpha2,
    )
    .await;

    if let Ok(ref user) = result {
        if let Ok(user_session) = UserSession::insert(&core_context, &user, true).await {
            let _ = start_user_session(&core_context, &user_session).await?;
        }

        if let Some(invitation_code) = invitation_code {
            let _ = invitation_code.delete(&core_context).await;
        }
    }

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_send_password_reset_code(
    username_or_email: String,
) -> Result<ActionFormResp<()>, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let result = User::get_by_username_or_email(&core_context, &username_or_email).await;

    let Ok(user) = result else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let result = UserPasswordReset::delete_and_insert(&core_context, &user).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_password_with_code(
    username_or_email: String,
    code: String,
    new_password: String,
) -> Result<ActionFormResp<()>, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let result = User::get_by_username_or_email(&core_context, &username_or_email).await;

    let Ok(user) = result else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let result = user
        .update_password_with_code(&core_context, &code, &new_password)
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_invitation_code_id(code: String) -> Result<Option<String>, ServerFnError> {
    if !require_no_authentication().await? {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(InvitationCode::get_by_code(&core_context, &code)
        .await
        .map(|i| i.id.to_string())
        .ok())
}
