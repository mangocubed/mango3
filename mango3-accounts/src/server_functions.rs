use leptos::prelude::*;

use mango3_leptos_utils::models::ActionFormResp;

#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, require_no_authentication};

#[server]
pub async fn attempt_to_send_password_reset_code(username_or_email: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let result = User::get_by_username_or_email(&core_context, &username_or_email).await;

    let Ok(user) = result else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let result = user.send_password_reset_confirmation_code(&core_context).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_password_with_code(
    username_or_email: String,
    code: String,
    new_password: String,
) -> Result<ActionFormResp, ServerFnError> {
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
