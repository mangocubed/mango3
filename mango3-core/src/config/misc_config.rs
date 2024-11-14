use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::extract_from_env;

#[derive(Deserialize, Serialize)]
pub struct MiscConfig {
    pub(crate) confirmation_code_length: i8,
    pub(crate) storage_path: String,
}

impl Default for MiscConfig {
    fn default() -> Self {
        Self {
            confirmation_code_length: 6,
            storage_path: "./storage".to_owned(),
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
