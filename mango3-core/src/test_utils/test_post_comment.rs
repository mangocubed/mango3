use crate::models::{Post, PostComment, User};
use crate::CoreContext;

use super::{fake_paragraph, insert_test_post, insert_test_user};

pub async fn insert_test_post_comment(
    core_context: &CoreContext,
    post: Option<&Post>,
    user: Option<&User>,
) -> PostComment {
    let post = if let Some(post) = post {
        post
    } else {
        &insert_test_post(core_context, None, None).await
    };
    let user = if let Some(user) = user {
        user
    } else {
        &insert_test_user(core_context).await
    };
    let content = fake_paragraph();

    PostComment::insert(core_context, &post, &user, &content)
        .await
        .ok()
        .unwrap()
}
