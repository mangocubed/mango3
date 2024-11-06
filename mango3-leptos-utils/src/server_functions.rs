use leptos::prelude::*;

use crate::models::UserResp;

#[server]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    crate::ssr::is_authenticated().await
}

#[server]
pub async fn get_current_user() -> Result<Option<UserResp>, ServerFnError> {
    let Some(user) = crate::ssr::extract_user().await? else {
        return Ok(None);
    };

    Ok(Some(UserResp::from_user(user).await))
}
