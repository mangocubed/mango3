use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{CursorPageResp, PostPreviewResp, WebsiteResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Post, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::expect_core_context;

#[server]
pub async fn get_websites(after: Option<String>) -> Result<CursorPageResp<WebsiteResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Website::paginate_by_created_at_desc(&core_context, &page_params, None, Some(true)).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_posts(after: Option<String>) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, Some(true)).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_posts_search(
    query: String,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::search(&core_context, &page_params, None, None, Some(true), &query).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}
