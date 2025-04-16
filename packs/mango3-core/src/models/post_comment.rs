use std::borrow::Cow;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::CoreContext;

use super::User;

#[derive(Clone)]
pub struct PostComment<'a> {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostComment<'_> {
    pub async fn content_html(&self) -> Cow<'_, str> {
        Cow::Owned(post_comment_content_html(self).await.unwrap_or_default())
    }

    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        crate::commands::get_user_by_id(core_context, self.user_id).await
    }
}

#[cached::proc_macro::io_cached(
    map_error = r##"|err| err"##,
    convert = r#"{ comment.id }"#,
    ty = "cached::AsyncRedisCache<Uuid, String>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_POST_COMMENT_CONTENT_HTML).await } "##
)]
pub(crate) async fn post_comment_content_html(comment: &PostComment<'_>) -> Result<String, cached::RedisCacheError> {
    Ok(crate::utils::parse_html(&comment.content, true))
}
