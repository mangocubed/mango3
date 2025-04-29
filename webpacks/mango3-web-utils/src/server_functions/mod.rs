#[cfg(feature = "with-dioxus")]
use dioxus::prelude::{server, server_fn, ServerFnError};
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;

#[cfg(feature = "with-dioxus")]
use crate::presenters::AppConfigPresenter;

#[cfg(all(not(feature = "with-dioxus"), feature = "image-upload"))]
mod image_upload;

#[cfg(all(not(feature = "with-dioxus"), feature = "image-upload"))]
pub use image_upload::attempt_to_upload_image;

#[cfg(feature = "with-dioxus")]
#[server]
pub async fn get_app_config() -> Result<AppConfigPresenter, ServerFnError> {
    let locale = crate::ssr::extract_locale().await?;

    Ok(AppConfigPresenter { locale })
}

#[cfg(all(not(feature = "with-dioxus"), feature = "current-user"))]
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

#[cfg(feature = "with-dioxus")]
#[server]
pub async fn set_language(language: unic_langid::LanguageIdentifier) -> Result<(), ServerFnError> {
    let session = crate::ssr::extract_session().await?;

    session.insert("language", &language.to_string()).await?;

    Ok(())
}
