use std::fs;
use std::path::Path;
use std::str::FromStr;

use image::imageops::FilterType;
use mime::{Mime, IMAGE, JPEG};
use sqlx::query_as;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::{BASIC_CONFIG, MISC_CONFIG};
use crate::CoreContext;

use super::User;

mod blob_insert;

#[derive(Clone)]
pub struct Blob {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub file_name: String,
    pub content_type: String,
    pub byte_size: i64,
    pub md5_checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Blob {
    pub fn default_path(&self) -> String {
        format!("{}/default{}", self.directory(), self.extension())
    }

    pub fn directory(&self) -> String {
        format!("{}/blobs/{}", MISC_CONFIG.storage_path, self.id)
    }

    pub fn extension(&self) -> String {
        let mime = self.mime();
        match (mime.type_(), mime.subtype()) {
            (IMAGE, JPEG) => ".jpg".to_owned(),
            (_, subtype) => format!(".{subtype}"),
        }
    }

    pub fn filename_without_extension(&self) -> String {
        self.file_name.split('.').collect::<Vec<&str>>()[0].to_string()
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid, user: Option<&User>) -> sqlx::Result<Self> {
        let user_id = user.map(|u| u.id);

        query_as!(
            Self,
            "SELECT * FROM blobs WHERE id = $1 AND ($2::uuid IS NULL OR user_id = $2) LIMIT 1",
            id,      // $1
            user_id  // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_multiple_by_id(core_context: &CoreContext, ids: Vec<Uuid>, user: Option<&User>) -> Vec<Self> {
        if ids.is_empty() {
            return vec![];
        }

        let user_id = user.map(|u| u.id);

        query_as!(
            Self,
            "SELECT * FROM blobs WHERE id = ANY($1) AND ($2::uuid IS NULL OR user_id = $2)",
            &ids,    // $1
            user_id  // $2
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
    }

    pub fn mime(&self) -> Mime {
        Mime::from_str(&self.content_type).unwrap()
    }

    pub fn variant_filename(&self, width: Option<u32>, height: Option<u32>, fill: Option<bool>) -> String {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.map(|f| if f { "_fill" } else { "" }).unwrap_or_default();

            return format!(
                "{}_{}x{}{}{}",
                self.filename_without_extension(),
                width,
                height,
                fill,
                self.extension()
            );
        }

        self.file_name.clone()
    }

    pub fn image_variant_path(&self, width: u32, height: u32, fill: bool) -> String {
        format!(
            "{}/{}x{}{}{}",
            self.directory(),
            width,
            height,
            if fill { "_fill" } else { "" },
            self.extension()
        )
    }

    pub fn read(&self, width: Option<u32>, height: Option<u32>, fill: Option<bool>) -> Option<Vec<u8>> {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.unwrap_or(false);

            let variant_path = self.image_variant_path(width, height, fill);

            if !Path::new(&variant_path).exists() {
                let mut image = image::open(self.default_path()).unwrap();
                image = if fill {
                    image.resize_to_fill(width, height, FilterType::Triangle)
                } else {
                    image.resize(width, height, FilterType::Triangle)
                };
                image.save(variant_path.clone()).unwrap();
            }

            return fs::read(variant_path).ok();
        }

        fs::read(self.default_path()).ok()
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.blob_url(self.id)
    }
}
