use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{FormResp, UserProfileResp};

#[cfg(feature = "ssr")]
use mango3_core::enums::ConfirmationCodeAction;
#[cfg(feature = "ssr")]
use mango3_core::models::Blob;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{
    expect_core_context, extract_confirmation_code, extract_i18n, extract_user, finish_and_delete_user_session,
    require_authentication, start_confirmation_code,
};

#[server]
pub async fn attempt_to_confirm_email(code: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = confirmation_code
        .confirm(
            &core_context.clone(),
            ConfirmationCodeAction::EmailConfirmation,
            &code,
            || {
                let core_context = core_context.clone();
                let user = user.clone();
                async move { user.confirm_email(&core_context).await.map(|_| ()) }
            },
        )
        .await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_logout() -> Result<(), ServerFnError> {
    if !require_authentication().await? {
        return Ok(());
    }

    let core_context = expect_core_context();

    finish_and_delete_user_session(&core_context).await?;

    Ok(())
}

#[server]
pub async fn attempt_to_send_email_confirmation_code() -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user.send_email_confirmation_code(&core_context).await;

    if let Ok(ref confirmation_code) = result {
        let _ = start_confirmation_code(&confirmation_code).await;
    }

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_email(email: String, password: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user.update_email(&core_context, &email, &password).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_password(
    current_password: String,
    new_password: String,
) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = user
        .update_password(&core_context, &current_password, &new_password)
        .await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_profile(
    display_name: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
    bio: String,
    avatar_image_blob_id: Option<String>,
) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?;

    let avatar_image_blob = if let Some(id) = avatar_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, user.as_ref(), None).await.ok()
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

    FormResp::new(&i18n, result)
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
