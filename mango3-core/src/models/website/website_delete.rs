use sqlx::query;

use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM websites WHERE id = $1", self.id)
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
    use crate::test_utils::{insert_test_website, setup_core_context};

    #[tokio::test]
    async fn should_delete_website() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = website.delete(&core_context).await;

        assert!(result.is_ok());
    }
}
