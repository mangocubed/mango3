use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use serde::{Deserialize, Serialize};

use crate::enums::UserRole;

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct MiscConfig {
    pub client_ip_source: String,
    default_user_role: String,
    pub(crate) confirmation_code_length: u8,
    pub(crate) font_path: String,
    image_ops_filter_type: String,
    pub(crate) invitation_code_length: u8,
    pub(crate) max_comment_content_length: u32,
    pub(crate) max_post_content_length: u32,
    pub(crate) storage_path: String,
    pub support_email_address: String,
}

impl Default for MiscConfig {
    fn default() -> Self {
        Self {
            client_ip_source: "XRealIp".to_owned(),
            confirmation_code_length: 6,
            default_user_role: "user".to_owned(),
            font_path: "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_owned(),
            image_ops_filter_type: "CatmullRom".to_owned(),
            invitation_code_length: 6,
            max_comment_content_length: 8192,
            max_post_content_length: 16384,
            #[cfg(not(test))]
            storage_path: "./storage".to_owned(),
            #[cfg(test)]
            storage_path: "./storage/tests".to_owned(),
            support_email_address: "support@mango3.local".to_owned(),
        }
    }
}

impl MiscConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("MISC_")
    }

    pub(crate) fn default_user_role(&self) -> UserRole {
        (&self.default_user_role).into()
    }

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
