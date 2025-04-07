use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{CursorPagePresenter, PostMinPresenter, PostPresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_client_ip, extract_user};

#[cfg(feature = "ssr")]
use super::current_website;

#[cfg(feature = "ssr")]
pub async fn current_post(id: Uuid) -> Result<Post, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Err(ServerFnError::new("website not found"));
    };

    let core_context = expect_core_context();

    Ok(mango3_core::commands::get_post_by_id(&core_context, id, Some(&website), None, Some(true)).await?)
}

#[server]
pub async fn get_posts(
    hashtag: Option<String>,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return mango3_web_utils::cursor_page_presenter!();
    };

    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };

    let hashtag = if let Some(name) = hashtag {
        let Ok(hashtag) = mango3_core::commands::get_hashtag_by_name(&core_context, &name).await else {
            return mango3_web_utils::cursor_page_presenter!();
        };

        Some(hashtag)
    } else {
        None
    };

    let page = mango3_core::commands::paginate_posts(
        &core_context,
        &page_params,
        Some(&website),
        None,
        hashtag.as_ref(),
        Some(true),
    )
    .await;

    mango3_web_utils::cursor_page_presenter!(&page)
}

#[server]
pub async fn get_posts_search(
    query: String,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return mango3_web_utils::cursor_page_presenter!();
    };

    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };
    let page =
        mango3_core::commands::search_posts(&core_context, &page_params, Some(&website), None, Some(true), &query)
            .await;

    mango3_web_utils::cursor_page_presenter!(&page)
}

#[server]
pub async fn get_post(slug: String) -> Result<Option<PostPresenter>, ServerFnError> {
    let Some(website) = current_website().await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let result = mango3_core::commands::get_post_by_slug(&core_context, &slug, &website).await;

    if let Ok(post) = result {
        let client_ip = extract_client_ip().await?;
        let user = extract_user().await?;

        let _ = mango3_core::commands::get_or_insert_post_view(&core_context, &post, user.as_ref(), &client_ip).await;

        Ok(Some(PostPresenter::from_model(&post).await))
    } else {
        Ok(None)
    }
}
