use cached::IOCachedAsync;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use super::{Post, User};

use crate::config::MISC_CONFIG;
use crate::enums::Input;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

mod post_comment_content;
mod post_comment_paginate;

use post_comment_content::POST_COMMENT_CONTENT_HTML;

#[derive(Clone)]
pub struct PostComment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostComment {
    fn cache_remove(&self) {
        POST_COMMENT_CONTENT_HTML
            .get()
            .map(|cache| cache.cache_remove(&self.id));
    }

    pub async fn count(core_context: &CoreContext, post: &Post) -> sqlx::Result<i64> {
        query!(
            "SELECT COUNT(*) FROM post_comments WHERE post_id = $1 LIMIT 1",
            post.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
    }

    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM post_comments WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())?;

        self.cache_remove();

        Ok(())
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid, user: Option<&User>) -> sqlx::Result<Self> {
        let user_id = user.map(|user| user.id);
        query_as!(
            Self,
            r#"SELECT id, post_id, user_id, content, created_at, updated_at
            FROM post_comments WHERE id = $1 AND ($2::uuid IS NULL OR user_id = $2) LIMIT 1"#,
            id,      // $1
            user_id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn insert(
        core_context: &CoreContext,
        post: &Post,
        user: &User,
        content: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        let content = content.trim();

        if validator.validate_presence(Input::Content, content) {
            validator.validate_length(
                Input::Content,
                content,
                Some(1),
                Some(MISC_CONFIG.max_comment_content_length),
            );
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        query_as!(
            Self,
            "INSERT INTO post_comments (post_id, user_id, content) VALUES ($1, $2, $3) RETURNING
                id, post_id, user_id, content, created_at, updated_at",
            post.id, // $1
            user.id, // $2
            content, // $3
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        User::get_by_id(core_context, self.user_id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_paragraph, insert_test_post, insert_test_user, setup_core_context};

    use super::PostComment;

    #[tokio::test]
    async fn should_count_post_comments() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = PostComment::count(&core_context, &post).await.unwrap();

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_insert_post_comment() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;
        let content = fake_paragraph();

        let result = PostComment::insert(&core_context, &post, &user, &content).await;

        assert!(result.is_ok());
    }
}
