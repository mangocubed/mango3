use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;

use mime::{Mime, IMAGE, JPEG};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use url::Url;

use crate::config::{BASIC_CONFIG, MISC_CONFIG};

#[derive(Clone, Deserialize, Serialize)]
pub struct Blob<'a> {
    pub id: Uuid,
    pub website_id: Option<Uuid>,
    pub user_id: Uuid,
    pub file_name: Cow<'a, str>,
    pub content_type: Cow<'a, str>,
    pub byte_size: i64,
    pub md5_checksum: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for Blob<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Blob<'_> {
    #[cfg(feature = "blob-is-removable")]
    pub async fn is_removable(&self, core_context: &crate::CoreContext) -> bool {
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

    pub fn default_path(&self) -> Cow<str> {
        Cow::Owned(format!("{}/default{}", self.directory(), self.extension()))
    }

    pub fn directory(&self) -> Cow<str> {
        Cow::Owned(format!("{}/blobs/{}", MISC_CONFIG.storage_path, self.id))
    }

    pub fn extension(&self) -> Cow<str> {
        let mime = self.mime();
        match (mime.type_(), mime.subtype()) {
            (IMAGE, JPEG) => Cow::Borrowed(".jpg"),
            (_, subtype) => Cow::Owned(format!(".{subtype}")),
        }
    }

    pub fn filename_without_extension(&self) -> &str {
        self.file_name.split('.').collect::<Vec<&str>>()[0]
    }

    pub fn mime(&self) -> Mime {
        Mime::from_str(&self.content_type).unwrap()
    }

    pub fn variant_filename(&self, width: Option<u16>, height: Option<u16>, fill: Option<bool>) -> Cow<str> {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.map(|f| if f { "_fill" } else { "" }).unwrap_or_default();

            return Cow::Owned(format!(
                "{}_{}x{}{}{}",
                self.filename_without_extension(),
                width,
                height,
                fill,
                self.extension()
            ));
        }

        Cow::Borrowed(&self.file_name)
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

    #[cfg(feature = "blob-read")]
    pub fn read(&self, width: Option<u16>, height: Option<u16>, fill: Option<bool>) -> Option<Vec<u8>> {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.unwrap_or(false);

            let variant_path = self.image_variant_path(width, height, fill);

            if !std::path::Path::new(&variant_path).exists() {
                use image::ImageDecoder;

                let mut image_decoder = image::ImageReader::open(self.default_path().to_string())
                    .expect("Could not get image")
                    .into_decoder()
                    .expect("Could not convert image into decoder");
                let orientation = image_decoder
                    .orientation()
                    .unwrap_or(image::metadata::Orientation::NoTransforms);
                let mut dynamic_image =
                    image::DynamicImage::from_decoder(image_decoder).expect("Could not get dynamic image");

                dynamic_image.apply_orientation(orientation);

                dynamic_image = if fill {
                    dynamic_image.resize_to_fill(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                } else {
                    dynamic_image.resize(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                };

                dynamic_image.save(variant_path.clone()).unwrap();
            }

            return std::fs::read(variant_path).ok();
        }

        std::fs::read(self.default_path().to_string()).ok()
    }

    pub fn url(&self) -> Url {
        BASIC_CONFIG.blob_url(self.id)
    }
}
