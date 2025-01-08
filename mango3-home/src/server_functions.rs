use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{CursorPageResp, HashtagResp, PostPreviewResp, WebsitePreviewResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Hashtag, Post, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::expect_core_context;

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<HashtagResp>, ServerFnError> {
    let core_context = expect_core_context();

    Ok(Hashtag::get_by_name(&core_context, &name)
        .await
        .map(|hashtag| (&hashtag).into())
        .ok())
}

#[server]
pub async fn get_posts(
    hashtag: Option<String>,
    first: u8,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first,
    };

    let hashtag = if let Some(name) = hashtag {
        let Ok(hashtag) = Hashtag::get_by_name(&core_context, &name).await else {
            return Ok(CursorPageResp::default());
        };

        Some(hashtag)
    } else {
        None
    };

    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, hashtag.as_ref(), Some(true)).await;

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

#[server]
pub async fn get_websites(
    first: u8,
    after: Option<String>,
) -> Result<CursorPageResp<WebsitePreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first,
    };
    let page = Website::paginate_by_created_at_desc(&core_context, &page_params, None, Some(true)).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_websites_search(
    query: String,
    after: Option<String>,
) -> Result<CursorPageResp<WebsitePreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Website::search(&core_context, &page_params, None, Some(true), &query).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}
