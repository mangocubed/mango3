use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::types::Uuid;

use crate::constants::*;
use crate::models::*;

#[cfg(feature = "all-blobs-by-ids")]
pub async fn all_blobs_by_ids<'a>(ids: Vec<Uuid>, website: Option<&Website>, user: Option<&User>) -> Vec<Blob<'a>> {
    if ids.is_empty() {
        return vec![];
    }

    futures::future::join_all(ids.iter().map(|id| get_blob_by_id(*id, website, user)))
        .await
        .iter()
        .filter_map(|result| result.as_ref().ok())
        .cloned()
        .collect()
}

#[cfg(feature = "delete-blob")]
pub async fn delete_blob(blob: &Blob<'_>) -> crate::utils::MutResult {
    use cached::IOCachedAsync;

    let db_pool = crate::db_pool().await;

    sqlx::query!("DELETE FROM blobs WHERE id = $1", blob.id)
        .execute(db_pool)
        .await?;

    let _ = std::fs::remove_dir_all(blob.directory().to_string());

    if let Some(cache) = GET_CACHED_BLOB_BY_ID.get() {
        let _ = cache.cache_remove(&blob.id).await;
    }

    crate::mut_success!()
}

#[cfg(feature = "delete-orphaned-blobs")]
pub async fn delete_orphaned_blobs() -> crate::utils::MutResult {
    let db_pool = crate::db_pool().await;

    let result = sqlx::query_as!(
        Blob,
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
    .fetch_all(db_pool)
    .await;

    if let Ok(blobs) = result {
        for blob in blobs {
            let _ = delete_blob(&blob).await;
        }
    }

    crate::mut_success!()
}

#[cfg(feature = "get-blob-by-id")]
#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, Blob>",
    create = r##" { crate::async_redis_cache!(PREFIX_GET_BLOB_BY_ID).await } "##
)]
async fn get_cached_blob_by_id(id: Uuid) -> sqlx::Result<Blob<'static>> {
    let db_pool = crate::db_pool().await;

    sqlx::query_as!(
        Blob,
        "SELECT * FROM blobs WHERE id = $1 LIMIT 1",
        id, // $1
    )
    .fetch_one(db_pool)
    .await
}

#[cfg(feature = "get-blob-by-id")]
pub async fn get_blob_by_id<'a>(id: Uuid, website: Option<&Website>, user: Option<&User>) -> sqlx::Result<Blob<'a>> {
    let blob = get_cached_blob_by_id(id).await?;
    if let Some(website) = website {
        if Some(website.id) != blob.website_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    if let Some(user) = user {
        if user.id != blob.user_id {
            return Err(sqlx::Error::RowNotFound);
        }
    }

    Ok(blob)
}

#[cfg(feature = "insert-blob")]
pub async fn insert_blob<'a>(
    core_context: &crate::CoreContext,
    user: &User,
    website: Option<&Website>,
    field: &mut multer::Field<'_>,
) -> crate::utils::MutResult<Blob<'a>> {
    use std::io::Write;

    use md5::Digest;

    let db_pool = crate::db_pool().await;
    let website_id = website.map(|w| w.id);
    let tmp_file_path = crate::config::MISC_CONFIG
        .storage_tmp_path()
        .join(Uuid::new_v4().to_string());
    let mut tmp_file = std::fs::File::create(&tmp_file_path)?;
    let mut byte_size = 0i64;
    let mut md5_hasher = md5::Md5::new();

    while let Some(chunk) = field.chunk().await.map_err(|_| crate::utils::MutError::default())? {
        tmp_file.write(&chunk)?;
        byte_size += chunk.len() as i64;
        md5_hasher.update(chunk);
    }

    #[cfg(feature = "website-storage")]
    {
        if let Some(website) = website {
            if website.available_storage(core_context).await.bytes() < byte_size {
                return crate::mut_error!();
            }
        }
    }

    let md5_checksum = format!("{:x}", md5_hasher.finalize());
    let content_type = field
        .content_type()
        .unwrap_or(&mime::APPLICATION_OCTET_STREAM)
        .to_string();

    let result = sqlx::query_as!(
        Blob,
        "SELECT * FROM blobs
        WHERE user_id = $1 AND website_id = $2 AND content_type = $3 AND byte_size = $4 AND md5_checksum = $5",
        user.id,      // $1
        website_id,   // $2
        content_type, // $3
        byte_size,    // $4
        md5_checksum, // $5
    )
    .fetch_one(db_pool)
    .await;

    if let Ok(ref blob) = result {
        let _ = std::fs::remove_file(tmp_file_path);

        return crate::mut_success!(blob.clone());
    }

    if !ALLOWED_FILE_TYPES.contains(&content_type.as_str()) {
        let _ = std::fs::remove_file(tmp_file_path);

        return crate::mut_error!();
    }

    let file_name = field.file_name().unwrap_or_default();

    let result = sqlx::query_as!(
        Blob,
        "INSERT INTO blobs (user_id, website_id, file_name, content_type, byte_size, md5_checksum)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *",
        user.id,      // $1
        website_id,   // $2
        file_name,    // $3
        content_type, // $4
        byte_size,    // $5
        md5_checksum, // $6
    )
    .fetch_one(db_pool)
    .await;

    if let Ok(ref blob) = result {
        let _ = std::fs::create_dir_all(blob.directory().to_string());
        let _ = std::fs::rename(&tmp_file_path, blob.default_path().to_string());
        let _ = std::fs::remove_file(tmp_file_path);
    }

    crate::mut_result!(result)
}

#[cfg(feature = "paginate-blobs")]
pub async fn paginate_blobs<'a>(
    core_context: &'a crate::CoreContext,
    cursor_page_params: &crate::utils::CursorPageParams,
    website: Option<&'a Website>,
    user: Option<&'a User>,
) -> crate::utils::CursorPage<Blob<'a>> {
    crate::cursor_page!(
        core_context,
        cursor_page_params,
        |node: Blob| node.id,
        move |_, after| async move { get_blob_by_id(after, website, user).await.ok() },
        move |core_context, cursor_resource, limit| async move {
            let website_id = website.map(|w| w.id);
            let user_id = user.map(|u| u.id);
            let (cursor_id, cursor_created_at) = cursor_resource
                .map(|c| (Some(c.id), Some(c.created_at)))
                .unwrap_or_default();

            sqlx::query_as!(
                Blob,
                r#"SELECT * FROM blobs
                WHERE ($1::uuid IS NULL OR website_id = $1) AND ($2::uuid IS NULL OR user_id = $2)
                    AND ($4::timestamptz IS NULL OR created_at < $4 OR (created_at = $4 AND id < $3))
                ORDER BY created_at DESC, id DESC LIMIT $5"#,
                website_id,        // $1
                user_id,           // $2
                cursor_id,         // $3
                cursor_created_at, // $4
                limit,             // $5
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        insert_test_blob, insert_test_post, insert_test_user, insert_test_website, setup_core_context,
    };
    use crate::utils::CursorPageParams;

    use super::{delete_blob, get_blob_by_id, paginate_blobs};

    #[tokio::test]
    async fn should_delete_blob() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let blob = insert_test_blob(&core_context, Some(&user), None).await;

        let result = delete_blob(&blob).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_blob_by_id() {
        let core_context = setup_core_context().await;
        let blob = insert_test_blob(&core_context, None, None).await;

        let result = get_blob_by_id(blob.id, None, None).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_blob_by_id_with_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let blob = insert_test_blob(&core_context, Some(&user), None).await;

        let result = get_blob_by_id(blob.id, None, Some(&user)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_blob_by_id_when_user_is_invalid() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let blob = insert_test_blob(&core_context, None, None).await;

        let result = get_blob_by_id(blob.id, None, Some(&user)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_zero_blobs() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;

        let cursor_page =
            paginate_blobs(&core_context, &CursorPageParams::default(), Some(&website), Some(&user)).await;

        assert!(cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_get_one_blob() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;
        let website = insert_test_website(&core_context, Some(&user)).await;
        insert_test_post(&core_context, Some(&website), Some(&user)).await;

        let cursor_page =
            paginate_blobs(&core_context, &CursorPageParams::default(), Some(&website), Some(&user)).await;

        assert_eq!(cursor_page.nodes.len(), 1);
    }
}
