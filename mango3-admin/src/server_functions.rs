use leptos::prelude::*;

#[cfg(feature = "ssr")]
use mango3_core::enums::UserRole;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::extract_user;

#[cfg(feature = "ssr")]
const ALLOWED_ROLES: [UserRole; 2] = [UserRole::Admin, UserRole::Superuser];

#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(false);
    };

    Ok(ALLOWED_ROLES.contains(&user.role))
}
