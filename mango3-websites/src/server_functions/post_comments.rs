use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, CursorPageResp, PostCommentResp};

#[cfg(feature = "ssr")]
use mango3_core::models::PostComment;
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, extract_user, require_authentication};

#[cfg(feature = "ssr")]
use super::posts::current_post;

#[server]
pub async fn attempt_to_create_post_comment(post_id: String, content: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();
    let core_context = expect_core_context();

    let result = PostComment::insert(&core_context, &post, &user, &content).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_post_comments(
    post_id: String,
    after: Option<String>,
) -> Result<CursorPageResp<PostCommentResp>, ServerFnError> {
    let post = current_post(post_id).await?;
    let core_context = expect_core_context();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };

    let page = PostComment::paginate_by_created_at_desc(&core_context, &page_params, Some(&post), None).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}
