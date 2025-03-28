use std::fmt::Display;
use std::str::FromStr;

use futures::future;
use mime::{Mime, IMAGE, JPEG};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

#[cfg(feature = "website_storage")]
use size::Size;
#[cfg(feature = "website_storage")]
use sqlx::query;

use crate::config::{BASIC_CONFIG, MISC_CONFIG};
use crate::models::{User, Website};
use crate::CoreContext;

mod blob_get;

#[cfg(feature = "blob_delete")]
mod blob_delete;
#[cfg(feature = "blob_insert")]
mod blob_insert;
#[cfg(feature = "blob_paginate")]
mod blob_paginate;
#[cfg(feature = "blob_read")]
mod blob_read;

#[derive(Clone, Deserialize, Serialize)]
pub struct Blob {
    pub id: Uuid,
    pub website_id: Option<Uuid>,
    pub user_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub byte_size: i64,
    pub md5_checksum: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Blob {
    pub async fn all_by_ids(
        core_context: &CoreContext,
        ids: Vec<Uuid>,
        website: Option<&Website>,
        user: Option<&User>,
    ) -> Vec<Self> {
        if ids.is_empty() {
            return vec![];
        }

        future::join_all(ids.iter().map(|id| Self::get_by_id(core_context, *id, website, user)))
            .await
            .iter()
            .filter_map(|result| result.as_ref().ok())
            .cloned()
            .collect()
    }

    #[cfg(feature = "blob_delete")]
    pub async fn is_removable(&self, core_context: &CoreContext) -> bool {
        sqlx::query!(
            "SELECT id
            FROM blobs AS b
            WHERE
                id = $1 AND (
                    (
                        website_id IS NULL AND (
                            SELECT id FROM users AS u WHERE u.id = b.user_id AND u.avatar_image_blob_id = b.id LIMIT 1
                        ) IS NULL
                    ) OR (
                        (
                            SELECT id FROM websites AS w
                            WHERE w.id = b.website_id AND (w.cover_image_blob_id = b.id OR w.icon_image_blob_id = b.id)
                            LIMIT 1
                        ) IS NULL AND (
                            SELECT id FROM posts AS p
                            WHERE
                                p.website_id = b.website_id AND (p.cover_image_blob_id = b.id OR b.id = ANY(p.blob_ids))
                            LIMIT 1
                        ) IS NULL
                    )
                )
            LIMIT 1",
            self.id
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok()
    }

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

    pub fn mime(&self) -> Mime {
        Mime::from_str(&self.content_type).unwrap()
    }

    pub fn variant_filename(&self, width: Option<u16>, height: Option<u16>, fill: Option<bool>) -> String {
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

    pub fn image_variant_path(&self, width: u16, height: u16, fill: bool) -> String {
        format!(
            "{}/{}x{}{}{}",
            self.directory(),
            width,
            height,
            if fill { "_fill" } else { "" },
            self.extension()
        )
    }

    #[cfg(feature = "website_storage")]
    pub async fn website_used_storage(core_context: &CoreContext, website: &Website) -> sqlx::Result<Size> {
        query!(
            "SELECT SUM(byte_size)::bigint AS total_size FROM blobs WHERE website_id = $1 LIMIT 1",
            website.id
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| Size::from_bytes(record.total_size.unwrap_or_default()))
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.blob_url(self.id)
    }
}
