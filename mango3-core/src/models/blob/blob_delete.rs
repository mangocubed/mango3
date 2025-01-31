use std::fs;

use futures::future;
use sqlx::query;

use crate::models::User;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Blob;

impl Blob {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query!("DELETE FROM blobs WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await;

        match result {
            Ok(_) => {
                let _ = fs::remove_dir_all(self.directory());

                Ok(())
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        let blobs = Self::all_by_user(core_context, user).await;

        future::join_all(blobs.iter().map(|b| b.delete(core_context))).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_blob, insert_test_user, setup_core_context};

    use super::Blob;

    #[tokio::test]
    async fn should_delete_all_blobs() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        insert_test_blob(&core_context, Some(&user), None).await;

        let result = Blob::delete_all(&core_context, &user).await;

        assert!(result.is_ok());
    }
}
