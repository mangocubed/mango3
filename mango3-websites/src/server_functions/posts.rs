use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{CursorPageResp, PostPreviewResp, PostResp};

#[cfg(feature = "ssr")]
use mango3_core::commands::{HashtagGet, PostViewInsert};
#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_client_ip, extract_user};
#[cfg(feature = "ssr")]
use mango3_utils::models::Hashtag;
#[cfg(feature = "ssr")]
use mango3_utils::models::PostView;

#[cfg(feature = "ssr")]
use super::current_website;

#[cfg(feature = "ssr")]
pub async fn current_post(id: String) -> Result<Post, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Err(ServerFnError::new("website not found"));
    };

    let core_context = expect_core_context();

    Ok(Post::get_by_id(&core_context, Uuid::try_parse(&id)?, Some(&website), None, Some(true)).await?)
}

#[server]
pub async fn get_posts(
    hashtag: Option<String>,
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

    let hashtag = if let Some(name) = hashtag {
        let Ok(hashtag) = Hashtag::get_by_name(&core_context, &name).await else {
            return Ok(CursorPageResp::default());
        };

        Some(hashtag)
    } else {
        None
    };

    let page = Post::paginate_by_created_at_desc(
        &core_context,
        &page_params,
        Some(&website),
        None,
        hashtag.as_ref(),
        Some(true),
    )
    .await;

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
