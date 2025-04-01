use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::Post;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::{BlobPresenter, HashtagPresenter, UserMinPresenter, WebsiteMinPresenter};

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPresenter {
    pub id: Uuid,
    pub user: UserMinPresenter,
    pub title: String,
    pub slug: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub cover_image_blob: Option<BlobPresenter>,
    pub blobs: Vec<BlobPresenter>,
    pub is_published: bool,
    pub url: Url,
    pub comments_count: i64,
    pub reactions_count: i64,
    pub views_count: i64,
    pub published_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,

    #[cfg(feature = "post-content-html")]
    pub content_html: String,
}

#[cfg(feature = "ssr")]
impl FromModel<Post> for PostPresenter {
    fn from_model(core_context: &CoreContext, post: &Post) -> impl std::future::Future<Output = Self> {
        async move {
            let user = UserMinPresenter::from_model(
                core_context,
                &post.user(core_context).await.expect("Could not get user"),
            )
            .await;
            let hashtags = futures::future::join_all(
                post.hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let cover_image_blob = if let Some(Ok(blob)) = post.cover_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };
            let blobs = futures::future::join_all(
                post.blobs(&core_context)
                    .await
                    .iter()
                    .map(|blob| BlobPresenter::from_model(core_context, &blob)),
            )
            .await;

            Self {
                id: post.id,
                user,
                title: post.title.clone(),
                slug: post.slug.clone(),
                hashtags,
                cover_image_blob,
                blobs,
                is_published: post.is_published(core_context).await,
                url: post.url(&core_context).await,
                comments_count: post.comments_count(&core_context).await,
                reactions_count: post.reactions_count(&core_context).await,
                views_count: post.views_count(&core_context).await,
                published_at: post.published_at,
                modified_at: post.modified_at,
                created_at: post.created_at,
                updated_at: post.updated_at,

                #[cfg(feature = "post-content-html")]
                content_html: post.content_html().await,
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PostMinPresenter {
    pub id: Uuid,
    pub website: WebsiteMinPresenter,
    pub user: UserMinPresenter,
    pub title: String,
    pub slug: String,
    pub content_preview_html: String,
    pub hashtags: Vec<HashtagPresenter>,
    pub cover_image_blob: Option<BlobPresenter>,
    pub is_published: bool,
    pub comments_count: i64,
    pub reactions_count: i64,
    pub views_count: i64,
    pub url: Url,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl FromModel<Post> for PostMinPresenter {
    fn from_model(core_context: &CoreContext, post: &Post) -> impl std::future::Future<Output = Self> {
        async move {
            let website = WebsiteMinPresenter::from_model(
                core_context,
                &post.website(&core_context).await.expect("Could not get website"),
            )
            .await;
            let user = UserMinPresenter::from_model(
                core_context,
                &post.user(core_context).await.expect("Could not get user"),
            )
            .await;
            let hashtags = futures::future::join_all(
                post.hashtags(&core_context)
                    .await
                    .iter()
                    .map(|hashtag| HashtagPresenter::from_model(core_context, hashtag)),
            )
            .await;
            let cover_image_blob = if let Some(Ok(blob)) = post.cover_image_blob(&core_context).await {
                Some(BlobPresenter::from_model(core_context, &blob).await)
            } else {
                None
            };

            Self {
                id: post.id,
                website,
                user,
                title: post.title.clone(),
                slug: post.slug.clone(),
                content_preview_html: post.content_preview_html().await,
                hashtags,
                cover_image_blob,
                is_published: post.is_published(core_context).await,
                comments_count: post.comments_count(&core_context).await,
                reactions_count: post.reactions_count(&core_context).await,
                views_count: post.views_count(&core_context).await,
                url: post.url(&core_context).await,
                modified_at: post.modified_at,
                created_at: post.created_at,
                updated_at: post.updated_at,
            }
        }
    }
}
