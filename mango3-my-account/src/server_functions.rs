use leptos::prelude::*;

use mango3_leptos_utils::models::ActionFormResp;

#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::*;

#[server]
pub async fn attempt_to_logout() -> Result<(), ServerFnError> {
    if !require_authentication().await? {
        return Ok(());
    }

    let core_context = expect_core_context();

    finish_user_session(&core_context).await?;

    Ok(())
}

#[server]
pub async fn attempt_to_update_password(
    current_password: String,
    new_password: String,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user
        .update_password(&core_context, &current_password, &new_password)
        .await;

    ActionFormResp::new(&i18n, result)
}
