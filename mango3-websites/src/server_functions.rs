use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::*;

#[cfg(feature = "ssr")]
use mango3_core::models::{NavigationItem, Post, PostView, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_client_ip, extract_host, extract_user};

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
pub async fn get_all_navigation_items() -> Result<Vec<NavigationItemResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(vec![]);
    };

    let core_context = expect_core_context();
    let items = NavigationItem::all_by_website(&core_context, &website).await;

    Ok(items.iter().map(|item| item.into()).collect())
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
pub async fn get_posts(after: Option<String>) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(CursorPageResp::default());
    };

    let core_context = expect_core_context();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, Some(&website), None, Some(true)).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_posts_search(
    query: String,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(CursorPageResp::default());
    };

    let core_context = expect_core_context();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Post::search(&core_context, &page_params, Some(&website), None, Some(true), &query).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_post(slug: String) -> Result<Option<PostResp>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let result = Post::get_by_slug(&core_context, &slug, &website).await;

    if let Ok(post) = result {
        let client_ip = extract_client_ip().await?;
        let user = extract_user().await?;

        let _ = PostView::insert(&core_context, &post, user.as_ref(), &client_ip).await;

        Ok(Some(PostResp::from_core(&core_context, &post).await))
    } else {
        Ok(None)
    }
}
