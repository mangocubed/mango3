use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Blob;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct BlobPresenter {
    pub id: Uuid,
    pub file_name: String,
    pub url: Url,

    #[cfg(feature = "blob-is-removable")]
    pub is_removable: bool,
}

#[cfg(feature = "ssr")]
impl FromModel<Blob> for BlobPresenter {
    #[allow(unused_variables)]
    fn from_model(core_context: &CoreContext, blob: &Blob) -> impl std::future::Future<Output = Self> {
        async {
            Self {
                id: blob.id,
                file_name: blob.file_name.clone(),
                url: blob.url(),

                #[cfg(feature = "blob-is-removable")]
                is_removable: blob.is_removable(core_context).await,
            }
        }
    }
}

impl BlobPresenter {
    pub fn variant_url(&self, width: u16, height: u16, fill: bool) -> Url {
        let mut variant_url = self.url.clone();

        variant_url.set_query(Some(&format!("width={width}&height={height}&fill={fill}")));

        variant_url
    }
}
