use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{CursorPagePresenter, MutPresenter, UserMinPresenter};

#[cfg(feature = "ssr")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "ssr")]
use mango3_core::enums::UserRole;
#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_user};

#[cfg(feature = "ssr")]
const ALLOWED_ROLES: [UserRole; 2] = [UserRole::Admin, UserRole::Superuser];

#[cfg(feature = "ssr")]
pub async fn require_admin() -> Result<bool, ServerFnError> {
    if !is_admin().await? {
        leptos_axum::redirect(BASIC_CONFIG.home_url().as_str());

        return Ok(false);
    }

    Ok(true)
}

#[server]
pub async fn attempt_to_disable_user(id: Uuid) -> Result<MutPresenter, ServerFnError> {
    if !require_admin().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let core_context = expect_core_context();
    let user = mango3_core::commands::get_user_by_id(&core_context, id).await?;

    let result = mango3_core::commands::disable_user(&core_context, &user).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn attempt_to_enable_user(id: Uuid) -> Result<MutPresenter, ServerFnError> {
    if !require_admin().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let core_context = expect_core_context();
    let user = mango3_core::commands::get_user_by_id(&core_context, id).await?;

    let result = mango3_core::commands::enable_user(&core_context, &user).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(false);
    };

    Ok(ALLOWED_ROLES.contains(&user.role))
}

#[server]
pub async fn get_users(after: Option<Uuid>) -> Result<CursorPagePresenter<UserMinPresenter>, ServerFnError> {
    if !require_admin().await? {
        return mango3_web_utils::cursor_page_presenter!();
    }

    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::commands::paginate_users(&core_context, &page_params).await;

    mango3_web_utils::cursor_page_presenter!(&page)
}
