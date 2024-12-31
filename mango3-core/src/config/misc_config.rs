use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct MiscConfig {
    pub client_ip_source: String,
    pub(crate) confirmation_code_length: u8,
    pub(crate) invitation_code_length: u8,
    pub(crate) max_post_content_length: u32,
    pub(crate) storage_path: String,
}

impl Default for MiscConfig {
    fn default() -> Self {
        Self {
            client_ip_source: "XRealIp".to_owned(),
            confirmation_code_length: 6,
            invitation_code_length: 6,
            max_post_content_length: 16384,
            #[cfg(not(test))]
            storage_path: "./storage".to_owned(),
            #[cfg(test)]
            storage_path: "./storage/tests".to_owned(),
        }
    }
}

impl MiscConfig {
    pub(crate) fn load() -> Self {
        extract_from_env("MISC_")
    }

    pub fn storage_tmp_path(&self) -> PathBuf {
        Path::new(&self.storage_path).join("tmp")
    }
}
