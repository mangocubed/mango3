use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{CursorPagePresenter, HashtagPresenter, PostMinPresenter, WebsiteMinPresenter};

#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::expect_core_context;

use crate::presenters::UserProfilePresenter;

#[server]
pub async fn get_hashtag(name: String) -> Result<Option<HashtagPresenter>, ServerFnError> {
    let core_context = expect_core_context();
    let result = mango3_core::commands::get_hashtag_by_name(&core_context, &name).await;

    if let Ok(hashtag) = result {
        Ok(Some(HashtagPresenter::from_model(&core_context, &hashtag).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_hashtag_posts(
    id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };

    let hashtag = mango3_core::commands::get_hashtag_by_id(&core_context, id).await?;

    let page =
        mango3_core::commands::paginate_posts(&core_context, &page_params, None, None, Some(&hashtag), Some(true))
            .await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}

#[server]
pub async fn get_posts(first: u8, after: Option<Uuid>) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };

    let page = mango3_core::commands::paginate_posts(&core_context, &page_params, None, None, None, Some(true)).await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}

#[server]
pub async fn get_posts_search(
    query: String,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::commands::search_posts(&core_context, &page_params, None, None, Some(true), &query).await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}

#[server]
pub async fn get_user(username: String) -> Result<Option<UserProfilePresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let result = mango3_core::commands::get_user_by_username(&core_context, &username).await;

    if let Ok(user) = result {
        Ok(Some(UserProfilePresenter::from_model(&core_context, &user).await))
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
    let user = mango3_core::commands::get_user_by_id(&core_context, id).await?;
    let page_params = CursorPageParams { after, first: 10 };

    let page =
        mango3_core::commands::paginate_posts(&core_context, &page_params, None, Some(&user), None, Some(true)).await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}

#[server]
pub async fn get_websites(
    first: u8,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<WebsiteMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first };
    let page = mango3_core::commands::paginate_websites(&core_context, &page_params, None, Some(true)).await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}

#[server]
pub async fn get_websites_search(
    query: String,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<WebsiteMinPresenter>, ServerFnError> {
    let core_context = expect_core_context();

    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::commands::search_websites(&core_context, &page_params, None, Some(true), &query).await;

    Ok(CursorPagePresenter::from_model(&core_context, &page).await)
}
