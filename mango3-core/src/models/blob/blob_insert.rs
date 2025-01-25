use std::fs;
use std::io::Write;

use lazy_static::lazy_static;
use md5::{Digest, Md5};
use mime::{APPLICATION_OCTET_STREAM, IMAGE_BMP, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use multer::Field;
use sqlx::query_as;
use sqlx::types::Uuid;

use crate::config::MISC_CONFIG;
use crate::models::{User, Website};
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Blob;

lazy_static! {
    static ref ALLOWED_TYPES: Vec<String> = vec![
        IMAGE_BMP.to_string(),
        IMAGE_GIF.to_string(),
        IMAGE_JPEG.to_string(),
        IMAGE_PNG.to_string(),
        "image/webp".to_owned(),
    ];
}

impl Blob {
    pub async fn insert(
        core_context: &CoreContext,
        user: &User,
        website: Option<&Website>,
        field: &mut Field<'_>,
    ) -> Result<Blob, ValidationErrors> {
        let website_id = website.map(|w| w.id);
        let tmp_file_path = MISC_CONFIG.storage_tmp_path().join(Uuid::new_v4().to_string());
        let mut tmp_file = fs::File::create(&tmp_file_path).map_err(|_| ValidationErrors::default())?;
        let mut byte_size = 0i64;
        let mut md5_hasher = Md5::new();

        while let Some(chunk) = field.chunk().await.map_err(|_| ValidationErrors::default())? {
            tmp_file.write(&chunk).map_err(|_| ValidationErrors::default())?;
            byte_size += chunk.len() as i64;
            md5_hasher.update(chunk);
        }

        let md5_checksum = format!("{:x}", md5_hasher.finalize());
        let content_type = field.content_type().unwrap_or(&APPLICATION_OCTET_STREAM).to_string();

        let result = query_as!(
            Self,
            "SELECT * FROM blobs
            WHERE user_id = $1 AND website_id = $2 AND content_type = $3 AND byte_size = $4 AND md5_checksum = $5",
            user.id,      // $1
            website_id,   // $2
            content_type, // $3
            byte_size,    // $4
            md5_checksum, // $5
        )
        .fetch_one(&core_context.db_pool)
        .await;

        if let Ok(blob) = result {
            let _ = fs::remove_file(tmp_file_path);
            return Ok(blob);
        }

        if !ALLOWED_TYPES.contains(&content_type.to_owned()) {
            let _ = fs::remove_file(tmp_file_path);
            return Err(ValidationErrors::default());
        }

        let file_name = field.file_name().unwrap_or_default();

        let result = query_as!(
            Self,
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
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(blob) => {
                let _ = fs::create_dir_all(blob.directory());
                let _ = fs::rename(&tmp_file_path, blob.default_path());
                let _ = fs::remove_file(tmp_file_path);

                Ok(blob)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
