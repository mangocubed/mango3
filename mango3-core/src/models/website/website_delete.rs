use sqlx::query;

use crate::models::User;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Website;

impl Website {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM websites WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        query!("DELETE FROM websites WHERE user_id = $1", user.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_user, insert_test_website, setup_core_context};

    use super::Website;

    #[tokio::test]
    async fn should_delete_all_websites() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        insert_test_website(&core_context, Some(&user)).await;

        let result = Website::delete_all(&core_context, &user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_delete_website() {
        let core_context = setup_core_context().await;
        let website = insert_test_website(&core_context, None).await;

        let result = website.delete(&core_context).await;

        assert!(result.is_ok());
    }
}
