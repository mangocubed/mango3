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
use mango3_leptos_utils::ssr::{expect_core_context, extract_host};

#[cfg(feature = "ssr")]
pub async fn current_website() -> Result<Option<Website>, ServerFnError> {
    let host = extract_host().await?;

    let Some(subdomain) = host.split(".").next() else {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(Website::get_by_subdomain(&core_context, subdomain).await.ok())
}

#[server]
pub async fn get_current_website() -> Result<Option<WebsiteResp>, ServerFnError> {
    if let Some(website) = current_website().await? {
        let core_context = expect_core_context();

        Ok(Some(WebsiteResp::from_core(&core_context, &website).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_posts(after: Option<String>) -> Result<PageResp<PostResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(PageResp::default());
    };

    let core_context = expect_core_context();
    let page_params = PageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, Some(&website), None, Some(true)).await;

    Ok(PageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_post(slug: String) -> Result<Option<PostResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let result = Post::get_by_slug(&core_context, &slug, &website).await;

    if let Ok(post) = result {
        Ok(Some(PostResp::from_core(&core_context, &post).await))
    } else {
        Ok(None)
    }
}
