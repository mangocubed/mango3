#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;

#[cfg(all(not(feature = "with-dioxus"), feature = "image-upload"))]
mod image_upload;

#[cfg(all(not(feature = "with-dioxus"), feature = "image-upload"))]
pub use image_upload::attempt_to_upload_image;

#[cfg(feature = "with-dioxus")]
#[server]
pub async fn get_basic_config() -> Result<crate::presenters::BasicConfigPresenter, ServerFnError> {
    (mango3_core::config::BASIC_CONFIG.into())
}

#[cfg(all(not(feature = "with-dioxus"), feature = "current-user"))]
#[server]
pub async fn get_current_user() -> Result<Option<crate::presenters::UserPresenter>, ServerFnError> {
    let Some(user) = crate::ssr::extract_user().await? else {
        return Ok(None);
    };

    use crate::presenters::FromModel;

    Ok(Some(crate::presenters::UserPresenter::from_model(&user).await))
}

#[cfg(not(feature = "with-dioxus"))]
#[server]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    crate::ssr::is_authenticated().await
}
