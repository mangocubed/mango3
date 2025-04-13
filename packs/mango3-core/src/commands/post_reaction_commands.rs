use crate::models::*;

#[cfg(feature = "delete-post-reaction")]
pub async fn delete_post_reaction(post_reaction: &PostReaction<'_>) -> crate::utils::MutResult {
    let db_pool = crate::db_pool().await;

    sqlx::query!("DELETE FROM post_reactions WHERE id = $1", post_reaction.id)
        .execute(db_pool)
        .await?;

    crate::mut_success!()
}

#[cfg(feature = "get-post-reaction-emojis-count")]
pub async fn get_post_reaction_emojis_count(post: &Post) -> sqlx::Result<Vec<(String, i64)>> {
    let db_pool = crate::db_pool().await;

    sqlx::query!(
        "SELECT emoji, COUNT(*) as count FROM post_reactions WHERE post_id = $1 GROUP BY emoji ORDER BY count DESC",
        post.id, // $1
    )
    .fetch_all(db_pool)
    .await
    .map(|records| {
        records
            .into_iter()
            .map(|record| (record.emoji, record.count.unwrap_or_default()))
            .collect()
    })
}

#[cfg(feature = "get-post-reaction-by-post-and-user")]
pub async fn get_post_reaction_by_post_and_user<'a>(post: &Post, user: &User) -> sqlx::Result<PostReaction<'a>> {
    let db_pool = crate::db_pool().await;

    sqlx::query_as!(
        PostReaction,
        "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
        post.id, // $1
        user.id, // $2
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "get-post-reactions-count")]
pub async fn get_post_reactions_count(post: &Post) -> i64 {
    let db_pool = crate::db_pool().await;

    sqlx::query!(
        "SELECT COUNT(*) FROM post_reactions WHERE post_id = $1 LIMIT 1",
        post.id, // $1
    )
    .fetch_one(db_pool)
    .await
    .map(|record| record.count.unwrap_or_default())
    .unwrap_or_default()
}

#[cfg(feature = "insert-or-update-post-reaction")]
pub async fn insert_or_update_post_reaction<'a>(
    post: &Post,
    user: &User,
    emoji: &str,
) -> crate::utils::MutResult<PostReaction<'a>> {
    let db_pool = crate::db_pool().await;
    let mut validator = crate::validator!();

    validator.custom_validation(crate::enums::Input::Emoji, crate::enums::InputError::IsInvalid, &|| {
        crate::constants::REACTION_EMOJIS.contains(&emoji)
    });

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    if let Ok(reaction) = get_post_reaction_by_post_and_user(post, user).await {
        let result = sqlx::query_as!(
            PostReaction,
            "UPDATE post_reactions SET emoji = $1 WHERE id = $2 RETURNING *",
            emoji,       // $1
            reaction.id, // $2
        )
        .fetch_one(db_pool)
        .await;

        return crate::mut_result!(result);
    };

    let result = sqlx::query_as!(
        PostReaction,
        "INSERT INTO post_reactions (post_id, user_id, emoji) VALUES ($1, $2, $3) RETURNING *",
        post.id, // $1
        user.id, // $2
        emoji,   // $3
    )
    .fetch_one(db_pool)
    .await;

    crate::mut_result!(result)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, setup_core_context};

    use super::{get_post_reaction_by_post_and_user, get_post_reactions_count, insert_or_update_post_reaction};

    #[tokio::test]
    async fn should_count_post_reactions() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = get_post_reactions_count(&post).await;

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_not_get_reaction_by_post_and_user() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = get_post_reaction_by_post_and_user(&post, &user).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_insert_or_update_post_reaction() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = insert_or_update_post_reaction(&post, &user, "🙂").await;

        assert!(result.is_ok());
    }
}
