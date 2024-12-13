use std::str::FromStr;

use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use super::{Post, User};

use crate::validator::ValidationErrors;
use crate::CoreContext;

pub struct PostView {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Option<Uuid>,
    pub ip_address: IpNetwork,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl PostView {
    pub async fn count(core_context: &CoreContext, post: &Post) -> sqlx::Result<i64> {
        query!(
            "SELECT COUNT(*) FROM post_views WHERE post_id = $1 LIMIT 1",
            post.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map(|record| record.count.unwrap_or_default())
    }

    pub async fn insert(
        core_context: &CoreContext,
        post: &Post,
        user: Option<&User>,
        ip_address: &str,
    ) -> Result<Self, ValidationErrors> {
        let user_id = user.map(|u| u.id);
        let ip_address = IpNetwork::from_str(ip_address).map_err(|_| ValidationErrors::default())?;

        if let Ok(view) = query_as!(
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

        query_as!(
            Self,
            "INSERT INTO post_views (post_id, user_id, ip_address) VALUES ($1, $2, $3) RETURNING *",
            post.id,    // $1
            user_id,    // $2
            ip_address, // $3
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_ipv4, insert_test_post, insert_test_user, setup_core_context};

    use super::PostView;

    #[tokio::test]
    async fn should_count_post_views() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None).await;

        let count = PostView::count(&core_context, &post).await.unwrap();

        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn should_insert_post_view() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None).await;
        let user = insert_test_user(&core_context).await;
        let ip_address = fake_ipv4();

        let result = PostView::insert(&core_context, &post, Some(&user), &ip_address).await;

        assert!(result.is_ok());
    }
}
