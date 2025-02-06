use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use sqlx::types::Uuid;

use crate::models::{User, Website};
use crate::{async_redis_cache, CoreContext};

use super::Blob;

impl Blob {
    pub async fn get_by_id(
        core_context: &CoreContext,
        id: Uuid,
        user: Option<&User>,
        website: Option<&Website>,
    ) -> sqlx::Result<Self> {
        let blob = get_blob_by_id(core_context, id).await?;

        if let Some(user) = user {
            if user.id != blob.user_id {
                return Err(sqlx::Error::RowNotFound);
            }
        }

        if let Some(website) = website {
            if website.id != blob.user_id {
                return Err(sqlx::Error::RowNotFound);
            }
        }

        Ok(blob)
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, Blob>",
    create = r##" { async_redis_cache("get_blob_by_id").await } "##
)]
pub(crate) async fn get_blob_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<Blob> {
    query_as!(
        Blob,
        "SELECT * FROM blobs WHERE id = $1 LIMIT 1",
        id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{insert_test_blob, insert_test_user, setup_core_context};

    use super::Blob;

    #[tokio::test]
    async fn should_get_blob_by_id() {
        let core_context = setup_core_context().await;
        let blob = insert_test_blob(&core_context, None, None).await;

        let result = Blob::get_by_id(&core_context, blob.id, None, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_blob_by_id_with_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let blob = insert_test_blob(&core_context, Some(&user), None).await;

        let result = Blob::get_by_id(&core_context, blob.id, Some(&user), None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_blob_by_id_when_user_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let blob = insert_test_blob(&core_context, None, None).await;

        let result = Blob::get_by_id(&core_context, blob.id, Some(&user), None).await;

        assert!(result.is_err());
    }
}
