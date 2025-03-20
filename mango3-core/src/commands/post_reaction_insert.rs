use std::future::Future;

use sqlx::query_as;

use mango3_utils::models::PostReaction;

use crate::constants::REACTION_EMOJIS;
use crate::enums::{Input, InputError};
use crate::models::{Post, User};
use crate::validator::{ValidationErrors, Validator};
use crate::CoreContext;

pub trait PostReactionInsert {
    fn insert_or_update(
        core_context: &CoreContext,
        post: &Post,
        user: &User,
        emoji: &str,
    ) -> impl Future<Output = Result<PostReaction, ValidationErrors>>;
}

impl PostReactionInsert for PostReaction {
    fn insert_or_update(
        core_context: &CoreContext,
        post: &Post,
        user: &User,
        emoji: &str,
    ) -> impl Future<Output = Result<Self, ValidationErrors>> {
        let mut validator = Validator::default();

        validator.custom_validation(Input::Emoji, InputError::IsInvalid, &|| {
            REACTION_EMOJIS.contains(&emoji)
        });

        async move {
            if !validator.is_valid {
                return Err(validator.errors);
            }

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
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, setup_core_context};

    use super::{PostReaction, PostReactionInsert};

    #[tokio::test]
    async fn should_insert_or_update_post_reaction() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;

        let result = PostReaction::insert_or_update(&core_context, &post, &user, "🙂").await;

        assert!(result.is_ok());
    }
}
