use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::PostComment;

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
impl FromModel<PostComment<'_>> for PostCommentPresenter {
    async fn from_model(post_comment: &PostComment<'_>) -> PostCommentPresenter {
        let core_context = crate::ssr::expect_core_context();
        let user =
            UserMinPresenter::from_model(&post_comment.user(&core_context).await.expect("Could not get user")).await;

        Self {
            id: post_comment.id,
            user,
            content_html: post_comment.content_html().await.to_string(),
            created_at: post_comment.created_at,
            updated_at: post_comment.updated_at,
        }
    }
}

#[cfg(all(feature = "ssr", feature = "post-comment-presenter"))]
impl FromModel<mango3_core::models::PostComment<'_>> for () {
    async fn from_model(_: &mango3_core::models::PostComment<'_>) -> Self {
        ()
    }
}
