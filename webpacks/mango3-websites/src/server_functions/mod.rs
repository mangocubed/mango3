use leptos::prelude::*;

use mango3_web_utils::presenters::{HashtagPresenter, NavigationItemPresenter, WebsitePresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_host};

mod post_comments;
mod post_reactions;
mod posts;

pub use post_comments::{get_post_comments, AttemptToCreatePostComment};
pub use post_reactions::*;
pub use posts::{get_post, get_posts, get_posts_search};

#[cfg(feature = "ssr")]
async fn current_website() -> Result<Option<Website>, ServerFnError> {
    let host = extract_host().await?;

    let Some(subdomain) = host.split(".").next() else {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(
        mango3_core::commands::get_website_by_subdomain(&core_context, subdomain)
            .await
            .ok(),
    )
}

#[server]
pub async fn get_all_navigation_items() -> Result<Vec<NavigationItemPresenter>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(vec![]);
    };

    let navigation_items = mango3_core::commands::all_navigation_items_by_website(&website).await;

    Ok(futures::future::join_all(
        navigation_items
            .iter()
            .map(|item| NavigationItemPresenter::from_model(&item)),
    )
    .await)
}

#[server]
pub async fn get_current_website() -> Result<Option<WebsitePresenter>, ServerFnError> {
    if let Some(website) = current_website().await? {
        Ok(Some(WebsitePresenter::from_model(&website).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<HashtagPresenter>, ServerFnError> {
    if current_website().await?.is_none() {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let result = mango3_core::commands::get_hashtag_by_name(&core_context, &name).await;

    if let Ok(hashtag) = result {
        Ok(Some(HashtagPresenter::from_model(&hashtag).await))
    } else {
        Ok(None)
    }
}
