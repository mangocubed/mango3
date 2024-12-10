use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use super::{Blob, Post};

use crate::validator::ValidationErrors;
use crate::CoreContext;

pub struct PostAttachment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub blob_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostAttachment {
    pub async fn all(core_context: &CoreContext, post: Option<&Post>) -> Vec<Self> {
        let post_id = post.map(|p| p.id);

        query_as!(
            Self,
            "SELECT * FROM post_attachments WHERE $1::uuid IS NULL OR post_id = $1",
            post_id // $1
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
    }

    pub async fn blob(&self, core_context: &CoreContext) -> sqlx::Result<Blob> {
        Blob::get_by_id(core_context, self.blob_id, None).await
    }

    pub async fn delete_all(core_context: &CoreContext, skip: Vec<Self>, post: &Post) -> Result<(), ValidationErrors> {
        query!(
            "DELETE FROM post_attachments WHERE id != ALL($1) AND post_id = $2",
            &skip.iter().map(|item| item.id).collect::<Vec<Uuid>>(), // $1
            post.id                                                  // $2
        )
        .execute(&core_context.db_pool)
        .await
        .map(|_| ())
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn insert(core_context: &CoreContext, post: &Post, blob: &Blob) -> Result<Self, ValidationErrors> {
        if let Ok(attachment) = query_as!(
            Self,
            "SELECT * FROM post_attachments WHERE post_id = $1 AND blob_id = $2 LIMIT 1",
            post.id, // $1
            blob.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        {
            return Ok(attachment);
        };

        query_as!(
            Self,
            "INSERT INTO post_attachments (post_id, blob_id) VALUES ($1, $2) RETURNING *",
            post.id, // $1
            blob.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn save_all(core_context: &CoreContext, post: &Post, blobs: Vec<Blob>) -> Result<(), ValidationErrors> {
        let mut skip_from_removal = vec![];

        for blob in blobs {
            let Ok(attachment) = PostAttachment::insert(core_context, post, &blob).await else {
                continue;
            };

            skip_from_removal.push(attachment);
        }

        let _ = Self::delete_all(core_context, skip_from_removal, post).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_blob, insert_test_post, insert_test_user, setup_core_context};

    use super::PostAttachment;

    #[tokio::test]
    async fn should_insert_post_attachment() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let post = insert_test_post(&core_context, Some(&user)).await;
        let blob = insert_test_blob(&core_context, Some(&user)).await;

        let result = PostAttachment::insert(&core_context, &post, &blob).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_all_post_attachments() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None).await;

        let all = PostAttachment::all(&core_context, Some(&post)).await;

        assert!(!all.is_empty())
    }

    #[tokio::test]
    async fn should_save_all_post_attachments() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let post = insert_test_post(&core_context, Some(&user)).await;
        let blob = insert_test_blob(&core_context, Some(&user)).await;

        let result = PostAttachment::save_all(&core_context, &post, vec![blob]).await;

        assert!(result.is_ok());
    }
}
