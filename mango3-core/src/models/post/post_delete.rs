use sqlx::query;

use crate::models::User;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM posts WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        query!("DELETE FROM posts WHERE user_id = $1", user.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, insert_test_user, setup_core_context};

    use super::Post;

    #[tokio::test]
    async fn should_delete_all_posts() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        insert_test_post(&core_context, None, Some(&user)).await;

        let result = Post::delete_all(&core_context, &user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_delete_post() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = post.delete(&core_context).await;

        assert!(result.is_ok());
    }
}
