use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::Hashtag;

#[derive(Clone, Deserialize, Serialize)]
pub struct HashtagResp {
    pub id: String,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl From<&Hashtag> for HashtagResp {
    fn from(hashtag: &Hashtag) -> Self {
        Self {
            id: hashtag.id.to_string(),
            name: hashtag.name.clone(),
        }
    }
}
