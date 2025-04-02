use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_web_utils::presenters::MutPresenter;

#[cfg(feature = "ssr")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "ssr")]
use mango3_core::enums::ConfirmationCodeAction;
#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_utils::models::{InvitationCode, UserSession};
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{
    expect_core_context, extract_confirmation_code, extract_i18n, finish_confirmation_code, require_no_authentication,
    start_confirmation_code, start_user_session,
};

#[server]
pub async fn attempt_to_confirm_login(code: String) -> Result<MutPresenter, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error_result!();
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return mango3_web_utils::mut_presenter_error_result!();
    };

    let core_context = expect_core_context();
    let user = confirmation_code.user(&core_context).await?;

    let result = mango3_core::commands::confirm_confirmation_code(
            &core_context.clone(),
            ConfirmationCodeAction::LoginConfirmation,
            &code,
            || {
                let core_context = core_context.clone();
                let user = user.clone();
                async move {
                    let result = mango3_core::commands::insert_user_session(&core_context, &user).await;

                    match result {
                        Ok(ref user_session) => {
                            let _ = start_user_session(&core_context, &user_session).await;
                            let _ = finish_confirmation_code().await;

                            Ok(())
                        }
                        Err(err) => Err(err),
                    }
                }
            },
        )
        .await;

    mango3_web_utils::mut_presenter_result!(result)
}

#[server]
pub async fn attempt_to_login(username_or_email: String, password: String) -> Result<FormResp<bool>, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let Ok(user) = User::authenticate(&core_context, &username_or_email, &password).await else {
        return FormResp::new_with_error(&i18n);
    };

    if user.email_is_confirmed() {
        let result = user.send_login_confirmation_code(&core_context).await;

        if let Ok(ref confirmation_code) = result {
            let _ = start_confirmation_code(&confirmation_code).await;

            return FormResp::new_with_data(&i18n, result, false);
        }
    } else {
        let result = UserSession::insert(&core_context, &user).await;

        if let Ok(ref user_session) = result {
            let _ = start_user_session(&core_context, &user_session).await;

            return FormResp::new_with_data(&i18n, result, true);
        }
    }

    FormResp::new_with_error(&i18n)
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
) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let invitation_code = if !BASIC_CONFIG.enable_register {
        let Some(id) = invitation_code_id else {
            return FormResp::new_with_error(&i18n);
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
        if let Ok(user_session) = UserSession::insert(&core_context, &user).await {
            let _ = start_user_session(&core_context, &user_session).await?;
        }

        if let Some(invitation_code) = invitation_code {
            let _ = invitation_code.delete(&core_context).await;
        }
    }

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_send_password_reset_code(username_or_email: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let result = User::get_by_username_or_email(&core_context, &username_or_email).await;

    let Ok(user) = result else {
        return FormResp::new_with_error(&i18n);
    };

    let result = user.send_password_reset_code(&core_context).await;

    if let Ok(ref confirmation_code) = result {
        let _ = start_confirmation_code(&confirmation_code).await;
    }

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_reset_password(code: String, new_password: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = confirmation_code.user(&core_context).await?;

    let result = confirmation_code
        .confirm(
            &core_context.clone(),
            ConfirmationCodeAction::PasswordReset,
            &code,
            || {
                let core_context = core_context.clone();
                let user = user.clone();
                let new_password = new_password.clone();
                async move {
                    let result = user.reset_password(&core_context, &new_password).await;

                    match result {
                        Ok(_) => {
                            let _ = finish_confirmation_code().await;

                            Ok(())
                        }
                        Err(err) => Err(err),
                    }
                }
            },
        )
        .await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_get_invitation_code_id(code: String) -> Result<FormResp<String>, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let result = InvitationCode::get_by_code(&core_context, &code).await;

    match result {
        Ok(invitation_code) => FormResp::new_with_data(&i18n, Ok(()), invitation_code.id.to_string()),
        Err(_) => FormResp::new_with_error(&i18n),
    }
}
