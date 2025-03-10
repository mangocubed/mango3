use leptos::prelude::*;

use crate::models::UserResp;

#[cfg(feature = "ssr")]
use crate::models::FromCore;
#[cfg(feature = "ssr")]
use crate::ssr::{expect_core_context, extract_user};

#[cfg(feature = "image_upload")]
mod image_upload;

#[cfg(feature = "image_upload")]
pub use image_upload::attempt_to_upload_image;

#[server]
pub async fn get_current_user() -> Result<Option<UserResp>, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(Some(UserResp::from_core(&core_context, &user).await))
}

#[server]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    crate::ssr::is_authenticated().await
}
