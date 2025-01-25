use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use serde_json::to_string_pretty;

use mango3_leptos_utils::models::BlobResp;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct EditPostResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub variables: String,
    pub cover_image_blob: Option<BlobResp>,
    pub blobs: Vec<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for EditPostResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            slug: post.slug.clone(),
            content: post.content.clone(),
            variables: to_string_pretty(&post.variables).unwrap_or_else(|_| "{}".to_owned()),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: post.is_published(core_context).await,
            blobs: post.blobs(&core_context).await.iter().map(|blob| blob.into()).collect(),
            url: post.url(&core_context).await.to_string(),
        }
    }
}
