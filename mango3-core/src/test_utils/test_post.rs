use crate::models::{Post, User};
use crate::CoreContext;

use super::{fake_paragraph, fake_sentence, fake_slug, insert_test_blob, insert_test_user, insert_test_website};

pub async fn insert_test_post(core_context: &CoreContext, user: Option<&User>) -> Post {
    let user = if let Some(user) = user {
        user
    } else {
        &insert_test_user(core_context).await
    };
    let website = insert_test_website(core_context, Some(&user)).await;
    let title = fake_sentence();
    let slug = fake_slug();
    let content = fake_paragraph();
    let blob = insert_test_blob(&core_context, Some(&user)).await;

    Post::insert(
        core_context,
        &website,
        &user,
        &title,
        &slug,
        &content,
        vec![blob.clone()],
        Some(&blob),
        true,
    )
    .await
    .ok()
    .unwrap()
}
