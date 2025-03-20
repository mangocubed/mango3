use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use futures::future;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobResp, HashtagResp, UserPreviewResp, WebsitePreviewResp};

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostResp {
    pub id: String,
    pub user: UserPreviewResp,
    pub title: String,
    pub slug: String,
    pub hashtags: Vec<HashtagResp>,
    pub cover_image_blob: Option<BlobResp>,
    pub blobs: Vec<BlobResp>,
    pub is_published: bool,
    pub url: String,
    pub views_count: i64,
    pub comments_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    #[cfg(feature = "post_content_html")]
    pub content_html: String,

    #[cfg(feature = "post_reaction_count")]
    pub reactions_count: i64,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        let cover_image_blob = if let Some(Ok(blob)) = post.cover_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };
        let blobs = future::join_all(
            post.blobs(&core_context)
                .await
                .iter()
                .map(|blob| BlobResp::from_core(core_context, blob)),
        )
        .await;

        Self {
            id: post.id.to_string(),
            user: UserPreviewResp::from_core(
                &core_context,
                &post.user(&core_context).await.expect("Could not get user"),
            )
            .await,
            title: post.title.clone(),
            slug: post.slug.clone(),

            hashtags: post
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            cover_image_blob,
            blobs,
            is_published: post.is_published(core_context).await,
            url: post.url(&core_context).await.to_string(),
            views_count: post.views_count(&core_context).await,
            comments_count: post.comments_count(&core_context).await,
            published_at: post.published_at,
            modified_at: post.modified_at,
            created_at: post.created_at,
            updated_at: post.updated_at,

            #[cfg(feature = "post_content_html")]
            content_html: post.content_html().await,

            #[cfg(feature = "post_reaction_count")]
            reactions_count: post.reactions_count(&core_context).await,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPreviewResp {
    pub id: String,
    pub website: WebsitePreviewResp,
    pub user: UserPreviewResp,
    pub title: String,
    pub slug: String,
    pub hashtags: Vec<HashtagResp>,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub views_count: i64,
    pub comments_count: i64,
    pub url: String,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    #[cfg(feature = "post_content_preview_html")]
    pub content_preview_html: String,

    #[cfg(feature = "post_reaction_count")]
    pub reactions_count: i64,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostPreviewResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        let cover_image_blob = if let Some(Ok(blob)) = post.cover_image_blob(&core_context).await {
            Some(BlobResp::from_core(core_context, &blob).await)
        } else {
            None
        };

        Self {
            id: post.id.to_string(),
            website: WebsitePreviewResp::from_core(
                &core_context,
                &post.website(&core_context).await.expect("Could not get website"),
            )
            .await,
            user: UserPreviewResp::from_core(
                &core_context,
                &post.user(&core_context).await.expect("Could not get user"),
            )
            .await,
            title: post.title.clone(),
            slug: post.slug.clone(),
            hashtags: post
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            cover_image_blob,
            is_published: post.is_published(core_context).await,
            views_count: post.views_count(&core_context).await,
            comments_count: post.comments_count(&core_context).await,
            url: post.url(&core_context).await.to_string(),
            modified_at: post.modified_at,
            created_at: post.created_at,
            updated_at: post.updated_at,

            #[cfg(feature = "post_content_preview_html")]
            content_preview_html: post.content_preview_html().await,

            #[cfg(feature = "post_reaction_count")]
            reactions_count: post.reactions_count(&core_context).await,
        }
    }
}
