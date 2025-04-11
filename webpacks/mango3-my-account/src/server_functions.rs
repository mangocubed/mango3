use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::MutPresenter;

#[cfg(feature = "ssr")]
use mango3_core::enums::ConfirmationCodeAction;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{
    expect_core_context, extract_confirmation_code, extract_user, finish_and_delete_user_session,
    require_authentication, start_confirmation_code,
};

use crate::presenters::EditUserProfilePresenter;

#[server]
pub async fn attempt_to_confirm_email(code: String) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let Some(confirmation_code) = extract_confirmation_code().await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = mango3_core::commands::confirm_confirmation_code(
        &core_context.clone(),
        &confirmation_code,
        ConfirmationCodeAction::EmailConfirmation,
        &code,
        || {
            let core_context = core_context.clone();
            let user = user.clone();
            async move { mango3_core::commands::confirm_user_email(&core_context, &user).await }
        },
    )
    .await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_logout() -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_success!();
    }

    let core_context = expect_core_context();

    finish_and_delete_user_session(&core_context).await?;

    mango3_web_utils::mut_presenter_success!()
}

#[server]
pub async fn attempt_to_send_email_confirmation_code() -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = mango3_core::commands::send_user_email_confirmation_code(&core_context, &user).await;

    if let Ok(ref success) = result {
        let _ = start_confirmation_code(&success.data).await;
    }

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_update_email(email: String, password: String) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = mango3_core::commands::update_user_email(&core_context, &user, &email, &password).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_update_password(
    current_password: String,
    new_password: String,
) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result =
        mango3_core::commands::update_user_password(&core_context, &user, &current_password, &new_password).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_update_profile(
    display_name: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
    bio: String,
    avatar_image_blob_id: Option<Uuid>,
) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.expect("Could not get user");

    let avatar_image_blob = if let Some(id) = avatar_image_blob_id {
        mango3_core::commands::get_blob_by_id(id, None, Some(&user)).await.ok()
    } else {
        None
    };

    let result = mango3_core::commands::update_user_profile(
        &core_context,
        &user,
        &display_name,
        &full_name,
        &birthdate,
        &country_alpha2,
        &bio,
        avatar_image_blob.as_ref(),
    )
    .await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn get_user_profile() -> Result<Option<EditUserProfilePresenter>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    };

    if let Some(user) = extract_user().await? {
        Ok(Some(EditUserProfilePresenter::from_model(&user).await))
    } else {
        Ok(None)
    }
}
