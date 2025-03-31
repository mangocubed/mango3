use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;

use mango3_web_utils::models::{FormResp, PostCommentResp};
use mango3_utils::models::CursorPage;

#[cfg(feature = "ssr")]
use mango3_core::models::PostComment;
#[cfg(feature = "ssr")]
use mango3_web_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n, extract_user, require_authentication};
#[cfg(feature = "ssr")]
use mango3_utils::models::CursorPageParams;

#[cfg(feature = "ssr")]
use super::posts::current_post;

#[server]
pub async fn attempt_to_create_post_comment(post_id: String, content: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return FormResp::new_with_error(&i18n);
    }

    let post = current_post(post_id).await?;
    let user = extract_user().await?.unwrap();
    let core_context = expect_core_context();

    let result = PostComment::insert(&core_context, &post, &user, &content).await;

    FormResp::new(&i18n, result)
}

#[server]
pub async fn get_post_comments(
    post_id: String,
    after: Option<Uuid>,
) -> Result<CursorPage<PostCommentResp>, ServerFnError> {
    let post = current_post(post_id).await?;
    let core_context = expect_core_context();
    let page_params = CursorPageParams { after, first: 10 };

    let page = PostComment::paginate_by_created_at_desc(&core_context, &page_params, Some(&post), None).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(
            page.nodes
                .iter()
                .map(|comment| PostCommentResp::from_core(&core_context, comment)),
        )
        .await,
    })
}
