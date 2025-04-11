use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;
#[cfg(feature = "ssr")]
use serde_json::to_string_pretty;

use mango3_web_utils::presenters::BlobPresenter;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct EditPostPresenter {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub variables: String,
    pub cover_image_blob: Option<BlobPresenter>,
    pub blobs: Vec<BlobPresenter>,
    pub is_published: bool,
    pub url: Url,
}

#[cfg(feature = "ssr")]
impl FromModel<Post> for EditPostPresenter {
    async fn from_model(post: &Post) -> Self {
        let core_context = mango3_web_utils::ssr::expect_core_context();
        let cover_image_blob = if let Some(Ok(blob)) = post.cover_image_blob().await {
            Some(BlobPresenter::from_model(&blob).await)
        } else {
            None
        };
        let blobs = future::join_all(post.blobs().await.iter().map(|blob| BlobPresenter::from_model(blob))).await;

        Self {
            id: post.id,
            title: post.title.clone(),
            slug: post.slug.clone(),
            content: post.content.clone(),
            variables: to_string_pretty(&post.variables).unwrap_or_else(|_| "{}".to_owned()),
            cover_image_blob,
            is_published: post.is_published(&core_context).await,
            blobs,
            url: post.url(&core_context).await,
        }
    }
}
