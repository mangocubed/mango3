use leptos::prelude::*;

#[cfg(feature = "ssr")]
#[cfg(feature = "image-upload")]
mod image_upload;

#[cfg(feature = "image-upload")]
pub use image_upload::attempt_to_upload_image;

#[cfg(feature = "current-user")]
#[server]
pub async fn get_current_user() -> Result<Option<crate::presenters::UserPresenter>, ServerFnError> {
    let Some(user) = crate::ssr::extract_user().await? else {
        return Ok(None);
    };

    use crate::presenters::FromModel;

    Ok(Some(crate::presenters::UserPresenter::from_model(&user).await))
}

#[server]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    crate::ssr::is_authenticated().await
}
