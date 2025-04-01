use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[cfg(feature = "blob-read")]
use image::imageops::FilterType;

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct MiscConfig {
    pub client_ip_source: String,
    pub(crate) confirmation_code_length: u8,
    pub(crate) font_path: String,
    image_ops_filter_type: String,
    pub(crate) invitation_code_length: u8,
    pub(crate) max_comment_content_length: u32,
    pub(crate) max_post_content_length: u32,
    pub(crate) storage_path: String,
}

impl Default for MiscConfig {
    fn default() -> Self {
        Self {
            client_ip_source: "XRealIp".to_owned(),
            confirmation_code_length: 6,
            font_path: "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_owned(),
            image_ops_filter_type: "CatmullRom".to_owned(),
            invitation_code_length: 6,
            max_comment_content_length: 8192,
            max_post_content_length: 16384,
            #[cfg(not(test))]
            storage_path: format!("{}/storage", env!("CARGO_MANIFEST_DIR")),
            #[cfg(test)]
            storage_path: format!("{}/storage/tests", env!("CARGO_MANIFEST_DIR")),
        }
    }
}

impl MiscConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("MISC_")
    }

    #[cfg(feature = "blob-read")]
    pub(crate) fn image_ops_filter_type(&self) -> FilterType {
        match self.image_ops_filter_type.as_str() {
            "CatmullRom" => FilterType::CatmullRom,
            "Gaussian" => FilterType::Gaussian,
            "Triangle" => FilterType::Triangle,
            "Lanczos3" => FilterType::Lanczos3,
            _ => FilterType::Nearest,
        }
    }

    pub fn storage_tmp_path(&self) -> PathBuf {
        Path::new(&self.storage_path).join("tmp")
    }
}
