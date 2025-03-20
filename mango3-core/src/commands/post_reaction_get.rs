use std::future::Future;

use sqlx::{query, query_as};

use mango3_utils::models::PostReaction;

use crate::models::{Post, User};
use crate::CoreContext;

pub trait PostReactionGet {
    fn get_emojis_count<'a>(
        core_context: &'a CoreContext,
        post: &'a Post,
    ) -> impl Future<Output = sqlx::Result<Vec<(String, i64)>>>;

    fn get_by_post_and_user<'a>(
        core_context: &'a CoreContext,
        post: &'a Post,
        user: &'a User,
    ) -> impl Future<Output = sqlx::Result<PostReaction>>;
}

impl PostReactionGet for PostReaction {
    fn get_emojis_count<'a>(
        core_context: &'a CoreContext,
        post: &'a Post,
    ) -> impl Future<Output = sqlx::Result<Vec<(String, i64)>>> {
        async {
            query!(
                "SELECT emoji, COUNT(*) as count FROM post_reactions WHERE post_id = $1 GROUP BY emoji ORDER BY count DESC",
                post.id, // $1
            )
            .fetch_all(&core_context.db_pool)
            .await
            .map(|records| {
                records
                    .into_iter()
                    .map(|record| (record.emoji, record.count.unwrap_or_default()))
                    .collect()
            })
        }
    }

    fn get_by_post_and_user<'a>(
        core_context: &'a CoreContext,
        post: &'a Post,
        user: &'a User,
    ) -> impl Future<Output = sqlx::Result<PostReaction>> {
        query_as!(
            Self,
            "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
            post.id, // $1
            user.id, // $2
        )
        .fetch_one(&core_context.db_pool)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, setup_core_context};

    use super::{PostReaction, PostReactionGet};

    #[tokio::test]
    async fn should_not_get_reaction_by_post_and_user() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = PostReaction::get_by_post_and_user(&core_context, &post, &user).await;

        assert!(result.is_err());
    }
}
