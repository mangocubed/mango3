use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::models::PostComment;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::UserPreviewResp;

#[cfg(feature = "ssr")]
use super::FromCore;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostCommentResp {
    pub id: String,
    pub user: UserPreviewResp,
    pub content_html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[async_trait]
impl FromCore<PostComment> for PostCommentResp {
    async fn from_core(core_context: &CoreContext, comment: &PostComment) -> Self {
        Self {
            id: comment.id.to_string(),
            user: UserPreviewResp::from_core(
                &core_context,
                &comment.user(&core_context).await.expect("Could not get user"),
            )
            .await,
            content_html: comment.content_html().await,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    }
}
