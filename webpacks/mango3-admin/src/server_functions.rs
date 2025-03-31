use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;

use mango3_utils::models::CursorPage;
use mango3_web_utils::models::FormResp;
use mango3_web_utils::models::UserPreviewResp;

#[cfg(feature = "ssr")]
use mango3_core::config::BASIC_CONFIG;
#[cfg(feature = "ssr")]
use mango3_core::enums::UserRole;
#[cfg(feature = "ssr")]
use mango3_core::models::User;
#[cfg(feature = "ssr")]
use mango3_utils::models::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n, extract_user};

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
pub async fn attempt_to_disable_user(id: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_admin().await? {
        return FormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let user = User::get_by_id(&core_context, Uuid::try_parse(&id)?).await?;

    let result = user.disable(&core_context).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_enable_user(id: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_admin().await? {
        return FormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let user = User::get_by_id(&core_context, Uuid::try_parse(&id)?).await?;

    let result = user.enable(&core_context).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(false);
    };

    Ok(ALLOWED_ROLES.contains(&user.role))
}

#[server]
pub async fn get_users(after: Option<Uuid>) -> Result<CursorPage<UserPreviewResp>, ServerFnError> {
    if !require_admin().await? {
        return Ok(CursorPage::default());
    }

    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };
    let page = User::paginate_by_username_asc(&core_context, &page_params).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|user| UserPreviewResp::from_core(&core_context, user)),
        )
        .await,
    })
}
