use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use futures::future;
use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageReader};
use mime::{Mime, IMAGE, JPEG};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::{BASIC_CONFIG, MISC_CONFIG};
use crate::models::{User, Website};
use crate::CoreContext;

mod blob_delete;
mod blob_get;
mod blob_insert;

#[derive(Clone, Deserialize, Serialize)]
pub struct Blob {
    pub id: Uuid,
    pub user_id: Uuid,
    pub website_id: Option<Uuid>,
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
        user: Option<&User>,
        website: Option<&Website>,
    ) -> Vec<Self> {
        if ids.is_empty() {
            return vec![];
        }

        future::join_all(ids.iter().map(|id| Self::get_by_id(core_context, *id, user, website)))
            .await
            .iter()
            .filter_map(|result| result.as_ref().ok())
            .cloned()
            .collect()
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

    pub fn read(&self, width: Option<u16>, height: Option<u16>, fill: Option<bool>) -> Option<Vec<u8>> {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.unwrap_or(false);

            let variant_path = self.image_variant_path(width, height, fill);

            if !Path::new(&variant_path).exists() {
                let mut image_decoder = ImageReader::open(self.default_path())
                    .expect("Could not get image")
                    .into_decoder()
                    .expect("Could not convert image into decoder");
                let orientation = image_decoder.orientation().unwrap_or(Orientation::NoTransforms);
                let mut dynamic_image = DynamicImage::from_decoder(image_decoder).expect("Could not get dynamic image");

                dynamic_image.apply_orientation(orientation);

                dynamic_image = if fill {
                    dynamic_image.resize_to_fill(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                } else {
                    dynamic_image.resize(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                };

                dynamic_image.save(variant_path.clone()).unwrap();
            }

            return fs::read(variant_path).ok();
        }

        fs::read(self.default_path()).ok()
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.blob_url(self.id)
    }
}
