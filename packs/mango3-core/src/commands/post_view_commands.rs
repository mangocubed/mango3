use crate::models::Post;
use crate::CoreContext;

#[cfg(feature = "get-post-views-count")]
pub async fn get_post_views_count(core_context: &CoreContext, post: &Post) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) FROM post_views WHERE post_id = $1 LIMIT 1",
        post.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await
    .map(|record| record.count.unwrap_or_default())
    .unwrap_or_default()
}

#[cfg(feature = "get-or-insert-post-view")]
pub async fn get_or_insert_post_view(
    core_context: &CoreContext,
    post: &Post,
    user: Option<&User>,
    ip_address: &str,
) -> MutResult<PostView> {
    let user_id = user.map(|u| u.id);

    let ip_address = IpNetwork::from_str(ip_address).map_err(|_| ValidationErrors::default())?;

    if let Ok(view) = sqlx::query_as!(
        Self,
        "SELECT * FROM post_views
            WHERE post_id = $1 AND (
                ($2::uuid IS NOT NULL AND user_id = $2) OR ($2 IS NULL AND user_id IS NULL AND ip_address = $3)
            ) LIMIT 1",
        post.id,    // $1
        user_id,    // $2
        ip_address, // $3
    )
    .fetch_one(&core_context.db_pool)
    .await
    {
        return Ok(view);
    };

    sqlx::query_as!(
        Self,
        "INSERT INTO post_views (post_id, user_id, ip_address) VALUES ($1, $2, $3) RETURNING *",
        post.id,    // $1
        user_id,    // $2
        ip_address, // $3
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_ipv4, insert_test_post, insert_test_user, setup_core_context};

    use super::{get_or_insert_post_view, get_post_views_count};

    #[tokio::test]
    async fn should_count_post_views() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;

        let count = get_post_views_count(&core_context, &post).await;

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_insert_post_view() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;
        let ip_address = fake_ipv4();

        let result = get_or_insert_post_view(&core_context, &post, Some(&user), &ip_address).await;

        assert!(result.is_ok());
    }
}
