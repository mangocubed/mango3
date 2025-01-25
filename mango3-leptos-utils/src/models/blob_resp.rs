use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::Blob;

#[derive(Clone, Deserialize, Serialize)]
pub struct BlobResp {
    pub id: String,
    pub url: String,
}

#[cfg(feature = "ssr")]
impl From<&Blob> for BlobResp {
    fn from(blob: &Blob) -> Self {
        Self {
            id: blob.id.to_string(),
            url: blob.url().to_string(),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Blob> for BlobResp {
    fn from(blob: Blob) -> Self {
        Self::from(&blob)
    }
}

impl BlobResp {
    pub fn variant_url(&self, width: u16, height: u16, fill: bool) -> String {
        format!("{}?width={width}&height={height}&fill={fill}", self.url)
    }
}
