use crate::models::Post;
use crate::CoreContext;

#[cfg(feature = "delete-post-reaction")]
pub async fn delete_post_reaction(core_context: &CoreContext, post_rection: &PostReaction) -> MutResult {
    sqlx::query!("DELETE FROM post_reactions WHERE id = $1", self.id)
        .execute(&core_context.db_pool)
        .await
}

#[cfg(feature = "get-post-reaction-emojis-count")]
pub async fn get_emojis_count<'a>(core_context: &'a CoreContext, post: &'a Post) -> sqlx::Result<Vec<(String, i64)>> {
    sqlx::query!(
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

#[cfg(feature = "get-post-reaction-by-post-and-user")]
pub async fn get_post_reaction_by_post_and_user<'a>(
    core_context: &'a CoreContext,
    post: &'a Post,
    user: &'a User,
) -> sqlx::Result<PostReaction> {
    sqlx::query_as!(
        Self,
        "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
        post.id, // $1
        user.id, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-post-reactions-count")]
pub async fn get_post_reactions_count(core_context: &CoreContext, post: &Post) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) FROM post_reactions WHERE post_id = $1 LIMIT 1",
        post.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map(|record| record.count.unwrap_or_default())
    .unwrap_or_default()
}

#[cfg(feature = "insert-or-update-post-reaction")]
pub async fn insert_or_update_post_reaction(
    core_context: &CoreContext,
    post: &Post,
    user: &User,
    emoji: &str,
) -> MutResult<PostReaction> {
    let mut validator = Validator::default();

    validator.custom_validation(Input::Emoji, InputError::IsInvalid, &|| {
        REACTION_EMOJIS.contains(&emoji)
    });

    async move {
        if !validator.is_valid {
            return Err(validator.errors);
        }

        if let Ok(reaction) = sqlx::query_as!(
            Self,
            "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
            post.id, // $1
            user.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        {
            return sqlx::query_as!(
                Self,
                "UPDATE post_reactions SET emoji = $1 WHERE id = $2 RETURNING *",
                emoji,       // $1
                reaction.id, // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .map_err(|_| ValidationErrors::default());
        };

        sqlx::query_as!(
            Self,
            "INSERT INTO post_reactions (post_id, user_id, emoji) VALUES ($1, $2, $3) RETURNING *",
            post.id, // $1
            user.id, // $2
            emoji,   // $3
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, setup_core_context};

    use super::{get_post_reaction_by_post_and_user, get_post_reactions_count, insert_or_update_post_reaction};

    #[tokio::test]
    async fn should_count_post_reactions() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = get_post_reactions_count(&core_context, &post).await;

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_not_get_reaction_by_post_and_user() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = get_post_reaction_by_post_and_user(&core_context, &post, &user).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_insert_or_update_post_reaction() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = insert_or_update_post_reaction(&core_context, &post, &user, "🙂").await;

        assert!(result.is_ok());
    }
}
