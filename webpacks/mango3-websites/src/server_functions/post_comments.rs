use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{CursorPagePresenter, MutPresenter, PostCommentPresenter};

#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_user, require_authentication};

#[cfg(feature = "ssr")]
use super::posts::current_post;

#[server]
pub async fn attempt_to_create_post_comment(post_id: Uuid, content: String) -> Result<MutPresenter, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!();
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();

    let result = mango3_core::commands::insert_post_comment(&post, &user, &content).await;

    mango3_web_utils::mut_presenter!(result)
}

#[server]
pub async fn get_post_comments(
    post_id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<PostCommentPresenter>, ServerFnError> {
    let post = current_post(post_id).await?;
    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };

    let page = mango3_core::commands::paginate_post_comments(&core_context, &page_params, Some(&post), None).await;

    mango3_web_utils::cursor_page_presenter!(&page)
}
