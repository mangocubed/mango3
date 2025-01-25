use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

#[cfg(feature = "ssr")]
use crate::ssr::render_handlebars;

use super::{BlobResp, HashtagResp, UserPreviewResp, WebsitePreviewResp};

#[cfg(feature = "ssr")]
use super::{parse_html, FromCore};

#[derive(Clone, Deserialize, Serialize)]
pub struct PostResp {
    pub id: String,
    pub user: UserPreviewResp,
    pub title: String,
    pub slug: String,
    pub content_html: String,
    pub hashtags: Vec<HashtagResp>,
    pub cover_image_blob: Option<BlobResp>,
    pub blobs: Vec<BlobResp>,
    pub is_published: bool,
    pub url: String,
    pub views_count: i64,
    pub comments_count: i64,
    pub reactions_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
        Self {
            id: post.id.to_string(),
            user: UserPreviewResp::from_core(
                &core_context,
                &post.user(&core_context).await.expect("Could not get user"),
            )
            .await,
            title: post.title.clone(),
            slug: post.slug.clone(),
            content_html: parse_html(
                &render_handlebars(&post.content, &post.variables).unwrap_or_default(),
                true,
            ),
            hashtags: post
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            blobs: post.blobs(&core_context).await.iter().map(|blob| blob.into()).collect(),
            is_published: post.is_published(core_context).await,
            url: post.url(&core_context).await.to_string(),
            views_count: post.views_count,
            comments_count: post.comments_count,
            reactions_count: post.reactions_count,
            published_at: post.published_at,
            modified_at: post.modified_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
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
    pub content_preview_html: String,
    pub hashtags: Vec<HashtagResp>,
    pub cover_image_blob: Option<BlobResp>,
    pub is_published: bool,
    pub views_count: i64,
    pub comments_count: i64,
    pub reactions_count: i64,
    pub url: String,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<Post> for PostPreviewResp {
    async fn from_core(core_context: &CoreContext, post: &Post) -> Self {
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
            content_preview_html: parse_html(&post.content_preview(), false),
            hashtags: post
                .hashtags(&core_context)
                .await
                .iter()
                .map(|hashtag| hashtag.into())
                .collect(),
            cover_image_blob: post
                .cover_image_blob(&core_context)
                .await
                .and_then(|result| result.ok())
                .map(|blob| blob.into()),
            is_published: post.is_published(core_context).await,
            views_count: post.views_count,
            comments_count: post.comments_count,
            reactions_count: post.reactions_count,
            url: post.url(&core_context).await.to_string(),
            modified_at: post.modified_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
