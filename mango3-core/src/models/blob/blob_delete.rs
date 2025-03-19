use std::fs;

use cached::IOCachedAsync;
use sqlx::{query, query_as};

use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::blob_get::GET_BLOB_BY_ID;
use super::Blob;

impl Blob {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM blobs WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map_err(|_| ValidationErrors::default())?;

        let _ = fs::remove_dir_all(self.directory());

        if let Some(cache) = GET_BLOB_BY_ID.get() {
            let _ = cache.cache_remove(&self.id).await;
        }

        Ok(())
    }

    pub async fn delete_all_orphaned(core_context: &CoreContext) -> Result<(), ValidationErrors> {
        let result = query_as!(
            Self,
            "SELECT *
            FROM blobs AS b
            WHERE
                website_id IS NULL AND (
                    SELECT id FROM users AS u WHERE u.avatar_image_blob_id = b.id LIMIT 1
                ) IS NULL AND (
                    SELECT id FROM websites AS w WHERE w.cover_image_blob_id = b.id OR w.icon_image_blob_id = b.id
                    LIMIT 1
                ) IS NULL AND (
                    SELECT id FROM posts AS p WHERE p.cover_image_blob_id = b.id OR b.id = ANY(p.blob_ids) LIMIT 1
                ) IS NULL
            LIMIT 1",
        )
        .fetch_all(&core_context.db_pool)
        .await;

        if let Ok(blobs) = result {
            for blob in blobs {
                let _ = blob.delete(core_context).await;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_blob, insert_test_user, setup_core_context};

    #[tokio::test]
    async fn should_delete_blob() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let blob = insert_test_blob(&core_context, Some(&user), None).await;

        let result = blob.delete(&core_context).await;

        assert!(result.is_ok());
    }
}
