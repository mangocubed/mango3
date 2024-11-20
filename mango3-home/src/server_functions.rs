use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{PageResp, PostResp, WebsiteResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Post, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::PageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::expect_core_context;

#[server]
pub async fn get_websites(after: Option<String>) -> Result<PageResp<WebsiteResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = PageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Website::paginate_by_created_at_desc(&core_context, &page_params, None, Some(true)).await;

    Ok(PageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_posts(after: Option<String>) -> Result<PageResp<PostResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = PageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, Some(true)).await;

    Ok(PageResp::from_core(&core_context, &page).await)
}