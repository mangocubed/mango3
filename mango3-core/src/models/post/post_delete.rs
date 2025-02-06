use sqlx::query;

use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Post;

impl Post {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM posts WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())?;

        self.cache_remove();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_post, setup_core_context};

    #[tokio::test]
    async fn should_delete_post() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let result = post.delete(&core_context).await;

        assert!(result.is_ok());
    }
}
