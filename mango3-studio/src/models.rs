use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use futures::future;

use mango3_leptos_utils::models::{BlobResp, PostAttachmentResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Page, Post};
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct EditPageResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub url: String,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Page> for EditPageResp {
    async fn from_core(core_context: &CoreContext, page: &Page) -> Self {
        Self {
            id: page.id.to_string(),
            title: page.title.clone(),
            slug: page.slug.clone(),
            content: page.content.clone(),
            cover_image_blob: page
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: page.is_published(core_context).await,
            url: page.url(&core_context).await.to_string(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EditPostResp {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub attachments: Vec<PostAttachmentResp>,
    pub cover_image_blob: Option<BlobResp>,
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
            attachments: future::join_all(
                post.attachments(core_context)
                    .await
                    .iter()
                    .map(|attachment| PostAttachmentResp::from_core(core_context, attachment)),
            )
            .await,
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
