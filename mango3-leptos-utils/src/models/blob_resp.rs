use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Blob;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct BlobResp {
    pub id: String,
    pub file_name: String,
    pub url: String,

    #[cfg(feature = "blob_delete")]
    pub is_removable: bool,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Blob> for BlobResp {
    async fn from_core(core_context: &CoreContext, blob: &Blob) -> Self {
        Self {
            id: blob.id.to_string(),
            file_name: blob.file_name.clone(),
            url: blob.url().to_string(),

            #[cfg(feature = "blob_delete")]
            is_removable: blob.is_removable(core_context).await,
        }
    }
}

impl BlobResp {
    pub fn variant_url(&self, width: u16, height: u16, fill: bool) -> String {
        format!("{}?width={width}&height={height}&fill={fill}", self.url)
    }
}
