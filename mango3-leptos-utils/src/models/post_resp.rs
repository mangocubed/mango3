use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::BlobResp;

#[cfg(feature = "ssr")]
use super::{parse_html, FromCore};

#[derive(Clone, Deserialize, Serialize)]
pub struct PostResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content_html: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            slug: post.slug.clone(),
            content_html: parse_html(&post.content),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: post.is_published(core_context).await,
            url: post.url(&core_context).await.to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPreviewResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content_preview_html: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostPreviewResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title.clone(),
            slug: post.slug.clone(),
            content_preview_html: parse_html(&post.content_preview()),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: post.is_published(core_context).await,
            url: post.url(&core_context).await.to_string(),
        }
    }
}
