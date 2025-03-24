use leptos::prelude::*;
use uuid::Uuid;

use mango3_leptos_utils::models::{CursorPageResp, PostPreviewResp, UserProfileResp, WebsitePreviewResp};
use mango3_utils::models::Hashtag;

#[cfg(feature = "ssr")]
use mango3_core::commands::HashtagGet;
#[cfg(feature = "ssr")]
use mango3_core::models::{Post, User, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::expect_core_context;

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<Hashtag>, ServerFnError> {
    let core_context = expect_core_context();

    Ok(Hashtag::get_by_name(&core_context, &name).await.ok())
}

#[server]
pub async fn get_hashtag_posts(
    id: Uuid,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };

    let hashtag = Hashtag::get_by_id(&core_context, id).await?;

    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, Some(&hashtag), Some(true)).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[server]
pub async fn get_posts(first: u8, after: Option<String>) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first,
    };

    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, None, Some(true)).await;

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
pub async fn get_user(username: String) -> Result<Option<UserProfileResp>, ServerFnError> {
    let core_context = expect_core_context();

    let result = User::get_by_username(&core_context, &username).await;

    if let Ok(user) = result {
        Ok(Some(UserProfileResp::from_core(&core_context, &user).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_user_posts(
    id: String,
    after: Option<String>,
) -> Result<CursorPageResp<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();
    let user = User::get_by_id(&core_context, Uuid::try_parse(&id)?).await?;
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };

    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, None, Some(&user), None, Some(true)).await;

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
