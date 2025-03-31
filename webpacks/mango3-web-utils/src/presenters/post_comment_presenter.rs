use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::PostComment;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

use super::UserMinPresenter;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostCommentPresenter {
    pub id: Uuid,
    pub user: UserMinPresenter,
    pub content_html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl FromModel<PostComment> for PostCommentPresenter {
    fn from_model(
        core_context: &CoreContext,
        post_comment: &PostComment,
    ) -> impl std::future::Future<Output = PostCommentPresenter> {
        let user = UserPreviewPresenter::from_model(
            core_context,
            post_comment.user(&core_context).await.expect("Could not get user"),
        )
        .await;

        Self {
            id: post_comment.id,
            user,
            content_html: post_comment.content_html().await,
            created_at: post_comment.created_at,
            updated_at: post_comment.updated_at,
        }
    }
}
