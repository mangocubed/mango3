use leptos::prelude::*;

use mango3_leptos_utils::models::{HashtagResp, NavigationItemResp, WebsiteResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Hashtag, NavigationItem, Website};
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_host};

mod post_comments;
mod posts;

pub use post_comments::{get_post_comments, AttemptToCreatePostComment};
pub use posts::{get_post, get_posts, get_posts_search};

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
pub async fn get_hashtag(name: String) -> Result<Option<HashtagResp>, ServerFnError> {
    if current_website().await?.is_none() {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(Hashtag::get_by_name(&core_context, &name)
        .await
        .map(|hashtag| (&hashtag).into())
        .ok())
}
