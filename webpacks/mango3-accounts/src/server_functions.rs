use uuid::Uuid;

use mango3_web_utils::prelude::*;
use mango3_web_utils::presenters::MutPresenter;

#[cfg(feature = "server")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "server")]
use mango3_core::enums::ConfirmationCodeAction;
#[cfg(feature = "server")]
use mango3_web_utils::ssr::{
    extract_confirmation_code, extract_locale, finish_confirmation_code, require_no_authentication,
    start_confirmation_code, start_user_session,
};

#[server]
pub async fn attempt_to_confirm_login(code: String) -> Result<MutPresenter, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let user = confirmation_code.user().await?;

    let result = mango3_core::commands::confirm_confirmation_code(
        &confirmation_code,
        ConfirmationCodeAction::LoginConfirmation,
        &code,
        || {
            let user = user.clone();
            async move {
                let result = mango3_core::commands::insert_user_session(&user).await;

                if let Ok(ref success) = result {
                    let _ = start_user_session(&success.data).await;
                    let _ = finish_confirmation_code().await;
                }

                result
            }
        },
    )
    .await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_login(
    username_or_email: String,
    password: String,
) -> Result<MutPresenter<bool>, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let Ok(user) = mango3_core::commands::authenticate_user(&username_or_email, &password).await else {
        return mango3_web_utils::mut_presenter_error!();
    };

    if user.data.email_is_confirmed() {
        let result = mango3_core::commands::send_user_login_confirmation_code(&user.data).await;

        if let Ok(ref confirmation_code) = result {
            let _ = start_confirmation_code(&confirmation_code.data).await;

            return mango3_web_utils::mut_presenter!(mango3_core::mut_success!(false));
        }
    } else {
        let result = mango3_core::commands::insert_user_session(&user.data).await;

        if let Ok(ref user_session) = result {
            let _ = start_user_session(&user_session.data).await;

            return mango3_web_utils::mut_presenter!(mango3_core::mut_success!(false));
        }
    }

    mango3_web_utils::mut_presenter_error!()
}

#[server]
pub async fn attempt_to_register(
    invitation_code_id: Option<Uuid>,
    username: String,
    email: String,
    password: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
) -> Result<MutPresenter, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let locale = extract_locale().await?;

    let invitation_code = if !BASIC_CONFIG.enable_register {
        let Some(id) = invitation_code_id else {
            return mango3_web_utils::mut_presenter_error!();
        };

        Some(mango3_core::commands::get_invitation_code_by_id(id).await?)
    } else {
        None
    };

    let result = mango3_core::commands::insert_user(
        &username,
        &email,
        &password,
        &full_name,
        &birthdate,
        locale.language,
        &country_alpha2,
    )
    .await;

    if let Ok(ref success_insert) = result {
        if let Ok(success) = mango3_core::commands::insert_user_session(&success_insert.data).await {
            let _ = start_user_session(&success.data).await?;
        }

        if let Some(invitation_code) = invitation_code {
            let _ = mango3_core::commands::delete_invitation_code(&invitation_code).await;
        }
    }

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_send_password_reset_code(username_or_email: String) -> Result<MutPresenter, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let result = mango3_core::commands::get_user_by_username_or_email(&username_or_email).await;

    let Ok(user) = result else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let result = mango3_core::commands::send_user_password_reset_code(&user).await;

    if let Ok(ref success_send) = result {
        let _ = start_confirmation_code(&success_send.data).await;
    }

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_reset_password(code: String, new_password: String) -> Result<MutPresenter, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let user = confirmation_code.user().await?;

    let result = mango3_core::commands::confirm_confirmation_code(
        &confirmation_code,
        ConfirmationCodeAction::PasswordReset,
        &code,
        || {
            let user = user.clone();
            let new_password = new_password.clone();
            async move {
                let result = mango3_core::commands::reset_user_password(&user, &new_password).await;

                if let Ok(_) = result {
                    let _ = finish_confirmation_code().await;
                }

                result
            }
        },
    )
    .await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_get_invitation_code_id(code: String) -> Result<MutPresenter<Uuid>, ServerFnError> {
    if !require_no_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let result = mango3_core::commands::get_invitation_code(&code).await;

    match result {
        Ok(invitation_code) => mango3_web_utils::mut_presenter!(mango3_core::mut_success!(invitation_code.id)),
        Err(_) => mango3_web_utils::mut_presenter_error!(),
    }
}
