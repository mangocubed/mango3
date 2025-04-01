use crate::models::Post;
use crate::CoreContext;

#[cfg(feature = "get-post-comments-count")]
pub async fn get_post_comments_count(core_context: &CoreContext, post: &Post) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) FROM post_comments WHERE post_id = $1 LIMIT 1",
        post.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map(|record| record.count.unwrap_or_default())
    .unwrap_or_default()
}
