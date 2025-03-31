use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Hashtag;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct HashtagPresenter {
    pub id: Uuid,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl FromModel<Hashtag> for HashtagPresenter {
    fn from_model(_core_context: &CoreContext, hashtag: &Hashtag) -> impl std::future::Future<Output = Self> {
        async {
            Self {
                id: hashtag.id,
                name: hashtag.name.clone(),
            }
        }
    }
}
