use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use super::{Post, User};

use crate::constants::REACTION_EMOJIS;
use crate::enums::{Input, InputError};
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

pub struct PostReaction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub emoji: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostReaction {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM post_reactions WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        query!("DELETE FROM post_reactions WHERE user_id = $1", user.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_emojis_count(core_context: &CoreContext, post: &Post) -> sqlx::Result<Vec<(String, i64)>> {
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

    pub async fn count(core_context: &CoreContext, post: &Post) -> i64 {
        query!(
            "SELECT COUNT(*) FROM post_reactions WHERE post_id = $1 LIMIT 1",
            post.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
        .unwrap_or_default()
    }

    pub async fn get_by_post_and_user(core_context: &CoreContext, post: &Post, user: &User) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
            post.id, // $1
            user.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn insert_or_update(
        core_context: &CoreContext,
        post: &Post,
        user: &User,
        emoji: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        validator.custom_validation(Input::Emoji, InputError::IsInvalid, &|| {
            REACTION_EMOJIS.contains(&emoji)
        });

        if let Ok(reaction) = query_as!(
            Self,
            "SELECT * FROM post_reactions WHERE post_id = $1 AND user_id = $2 LIMIT 1",
            post.id, // $1
            user.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        {
            return query_as!(
                Self,
                "UPDATE post_reactions SET emoji = $1 WHERE id = $2 RETURNING *",
                emoji,       // $1
                reaction.id, // $2
            )
            .fetch_one(&core_context.db_pool)
            .await
            .map_err(|_| ValidationErrors::default());
        };

        query_as!(
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

    use super::PostReaction;

    #[tokio::test]
    async fn should_count_post_reactions() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = PostReaction::count(&core_context, &post).await;

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_not_get_reaction_by_post_and_user() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = PostReaction::get_by_post_and_user(&core_context, &post, &user).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_insert_or_update_post_reaction() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = PostReaction::insert_or_update(&core_context, &post, &user, "🙂").await;

        assert!(result.is_ok());
    }
}
