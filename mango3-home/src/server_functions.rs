use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;

use mango3_leptos_utils::models::{PostPreviewResp, UserProfileResp, WebsitePreviewResp};
use mango3_utils::models::{CursorPage, Hashtag};

#[cfg(feature = "ssr")]
use mango3_core::commands::HashtagGet;
#[cfg(feature = "ssr")]
use mango3_core::models::{Post, User, Website};
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::expect_core_context;
#[cfg(feature = "ssr")]
use mango3_utils::models::CursorPageParams;

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<Hashtag>, ServerFnError> {
    let core_context = expect_core_context();

    Ok(Hashtag::get_by_name(&core_context, &name).await.ok())
}

#[server]
pub async fn get_hashtag_posts(id: Uuid, after: Option<Uuid>) -> Result<CursorPage<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };

    let hashtag = Hashtag::get_by_id(&core_context, id).await?;

    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, Some(&hashtag), Some(true)).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|post| PostPreviewResp::from_core(&core_context, post)),
        )
        .await,
    })
}

#[server]
pub async fn get_posts(first: u8, after: Option<Uuid>) -> Result<CursorPage<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };

    let page = Post::paginate_by_created_at_desc(&core_context, &page_params, None, None, None, Some(true)).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|post| PostPreviewResp::from_core(&core_context, post)),
        )
        .await,
    })
}

#[server]
pub async fn get_posts_search(
    query: String,
    after: Option<Uuid>,
) -> Result<CursorPage<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = Post::search(&core_context, &page_params, None, None, Some(true), &query).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|post| PostPreviewResp::from_core(&core_context, post)),
        )
        .await,
    })
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
pub async fn get_user_posts(id: String, after: Option<Uuid>) -> Result<CursorPage<PostPreviewResp>, ServerFnError> {
    let core_context = expect_core_context();
    let user = User::get_by_id(&core_context, Uuid::try_parse(&id)?).await?;
    let page_params = CursorPageParams { after, first: 10 };

    let page =
        Post::paginate_by_created_at_desc(&core_context, &page_params, None, Some(&user), None, Some(true)).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|post| PostPreviewResp::from_core(&core_context, post)),
        )
        .await,
    })
}

#[server]
pub async fn get_websites(first: u8, after: Option<Uuid>) -> Result<CursorPage<WebsitePreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };
    let page = Website::paginate_by_created_at_desc(&core_context, &page_params, None, Some(true)).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|website| WebsitePreviewResp::from_core(&core_context, website)),
        )
        .await,
    })
}

#[server]
pub async fn get_websites_search(
    query: String,
    after: Option<Uuid>,
) -> Result<CursorPage<WebsitePreviewResp>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = Website::search(&core_context, &page_params, None, Some(true), &query).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|website| WebsitePreviewResp::from_core(&core_context, website)),
        )
        .await,
    })
}
