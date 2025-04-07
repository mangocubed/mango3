use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Hashtag;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct HashtagPresenter {
    pub id: Uuid,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl FromModel<Hashtag> for HashtagPresenter {
    async fn from_model(hashtag: &Hashtag) -> Self {
        Self {
            id: hashtag.id,
            name: hashtag.name.clone(),
        }
    }
}
