use sqlx::query;

use mango3_utils::models::PostReaction;

use crate::models::Post;
use crate::CoreContext;

pub(crate) trait PostReactionCount {
    async fn count(core_context: &CoreContext, post: &Post) -> i64;
}

impl PostReactionCount for PostReaction {
    async fn count(core_context: &CoreContext, post: &Post) -> i64 {
        query!(
            "SELECT COUNT(*) FROM post_reactions WHERE post_id = $1 LIMIT 1",
            post.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
        .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, setup_core_context};

    use super::{PostReaction, PostReactionCount};

    #[tokio::test]
    async fn should_count_post_reactions() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = PostReaction::count(&core_context, &post).await;

        assert_eq!(count, 0);
    }
}
