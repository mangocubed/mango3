use std::fs;
use std::io::Write;

use lazy_static::lazy_static;
use md5::{Digest, Md5};
use mime::{APPLICATION_OCTET_STREAM, IMAGE_BMP, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use multer::Field;
use sqlx::query_as;

use crate::models::User;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::Blob;

lazy_static! {
    static ref ALLOWED_TYPES: Vec<String> = vec![
        IMAGE_BMP.to_string(),
        IMAGE_GIF.to_string(),
        IMAGE_JPEG.to_string(),
        IMAGE_PNG.to_string()
    ];
}

impl Blob {
    pub async fn insert(
        core_context: &CoreContext,
        user: &User,
        field: &mut Field<'_>,
    ) -> Result<Blob, ValidationErrors> {
        let Some(chunk) = field.chunk().await.map_err(|_| ValidationErrors::default())? else {
            return Err(ValidationErrors::default());
        };

        let md5_hasher = Md5::digest(&chunk);
        let md5_checksum = format!("{:x}", md5_hasher);

        let content_type = field.content_type().unwrap_or(&APPLICATION_OCTET_STREAM).to_string();

        let byte_size = chunk.len() as i64;

        let result = query_as!(
            Self,
            "SELECT * FROM blobs WHERE user_id = $1 AND content_type = $2 AND byte_size = $3 AND md5_checksum = $4",
            user.id,      // $1
            content_type, // $2
            byte_size,    // $3
            md5_checksum, // $4
        )
        .fetch_one(&core_context.db_pool)
        .await;

        if let Ok(blob) = result {
            return Ok(blob);
        }

        if !ALLOWED_TYPES.contains(&content_type.to_owned()) {
            return Err(ValidationErrors::default());
        }

        let file_name = field.file_name().unwrap_or_default();

        let result = query_as!(
            Self,
            "INSERT INTO blobs (user_id, file_name, content_type, byte_size, md5_checksum) VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
            user.id,      // $1
            file_name,    // $2
            content_type, // $3
            byte_size,    // $4
            md5_checksum, // $5
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(blob) => {
                let _ = fs::create_dir_all(blob.directory());
                let mut file = fs::File::create(blob.default_path()).unwrap();
                file.write_all(&chunk).unwrap();

                Ok(blob)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }
}
