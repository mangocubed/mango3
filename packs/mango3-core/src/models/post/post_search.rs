use sqlx::query_as;

use mango3_utils::models::{CursorPage, CursorPageParams};

use crate::create_cursor_page;
use crate::models::{User, Website};
use crate::CoreContext;

use super::Post;

impl Post {}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, insert_test_website, setup_core_context};

    use super::{CursorPageParams, Post};

    #[tokio::test]
    async fn should_get_zero_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page = Post::search(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            "",
        )
        .await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_post() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        let post = insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page = Post::search(
            &core_context,
            &CursorPageParams::default(),
            Some(&website),
            Some(&user),
            None,
            &post.title,
        )
        .await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
