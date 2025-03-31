use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;

use mango3_web_utils::presenters::{CursorPagePresenter, HashtagPresenter, PostMinPresenter, WebsiteMinPresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::{CursorPage, CursorPageParams, Post, User, Website};
#[cfg(feature = "ssr")]
use mango3_web_utils::models::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::expect_core_context;

use crate::presenters::UserProfilePresenter;

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<HashtagPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    Ok(mango3_core::get_hashtags_by_name!(&core_context, &name).await.ok())
}

#[server]
pub async fn get_hashtag_posts(
    id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };

    let hashtag = mango3_core::get_hashtags_by_id!(&core_context, id).await?;

    let page = mango3_core::paginate_posts_by_created_at_desc!(
        &core_context,
        &page_params,
        None,
        None,
        Some(&hashtag),
        Some(true),
    )
    .await;

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
pub async fn get_posts(first: u8, after: Option<Uuid>) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };

    let page =
        mango3_core::paginate_posts_by_created_at_desc!(&core_context, &page_params, None, None, None, Some(true))
            .await;

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
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::search_posts!(&core_context, &page_params, None, None, Some(true), &query).await;

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
pub async fn get_user(username: String) -> Result<Option<UserProfilePresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let result = mango3_core::get_user_by_username!(&core_context, &username).await;

    if let Ok(user) = result {
        Ok(Some(UserProfileResp::from_core(&core_context, &user).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_user_posts(
    id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();
    let user = User::get_by_id(&core_context, id).await?;
    let page_params = CursorPageParams { after, first: 10 };

    let page = mango3_core::paginate_posts_by_created_at_desc!(
        &core_context,
        &page_params,
        None,
        Some(&user),
        None,
        Some(true)
    )
    .await;

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
pub async fn get_websites(
    first: u8,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<WebsiteMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };
    let page = mango3_core::paginate_websites_by_created_at_desc!(&core_context, &page_params, None, Some(true)).await;

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
) -> Result<CursorPagePresenter<WebsiteMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::search_websites!(&core_context, &page_params, None, Some(true), &query).await;

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
