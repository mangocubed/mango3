use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, UserProfileResp};

#[cfg(feature = "ssr")]
use mango3_core::models::Blob;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::*;

#[server]
pub async fn attempt_to_confirm_email(code: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user.confirm_email(&core_context, &code).await;

    ActionFormResp::new(&i18n, result)
}

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
pub async fn attempt_to_send_email_confirmation_code() -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user.send_email_confirmation_code(&core_context).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_email(email: String, password: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user.update_email(&core_context, &email, &password).await;

    ActionFormResp::new(&i18n, result)
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

#[server]
pub async fn attempt_to_update_profile(
    display_name: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
    bio: String,
    avatar_image_blob_id: Option<String>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?;

    let avatar_image_blob = if let Some(id) = avatar_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, user.as_ref()).await.ok()
    } else {
        None
    };

    let result = user
        .unwrap()
        .update_profile(
            &core_context,
            &display_name,
            &full_name,
            &birthdate,
            &country_alpha2,
            &bio,
            avatar_image_blob.as_ref(),
        )
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_user_profile() -> Result<Option<UserProfileResp>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    };

    if let Some(user) = extract_user().await? {
        let core_context = expect_core_context();

        Ok(Some(UserProfileResp::from_core(&core_context, &user).await))
    } else {
        Ok(None)
    }
}
