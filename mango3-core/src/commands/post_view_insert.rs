use std::future::Future;
use std::str::FromStr;

use ipnetwork::IpNetwork;
use sqlx::query_as;

use mango3_utils::models::PostView;

use crate::models::{Post, User};
use crate::validator::ValidationErrors;
use crate::CoreContext;

pub trait PostViewInsert {
    fn insert(
        core_context: &CoreContext,
        post: &Post,
        user: Option<&User>,
        ip_address: &str,
    ) -> impl Future<Output = Result<PostView, ValidationErrors>>;
}

impl PostViewInsert for PostView {
    fn insert(
        core_context: &CoreContext,
        post: &Post,
        user: Option<&User>,
        ip_address: &str,
    ) -> impl Future<Output = Result<PostView, ValidationErrors>> {
        let user_id = user.map(|u| u.id);

        async move {
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
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_ipv4, insert_test_post, insert_test_user, setup_core_context};

    use super::{PostView, PostViewInsert};

    #[tokio::test]
    async fn should_insert_post_view() {
        let core_context = setup_core_context().await;
        let post = insert_test_post(&core_context, None, None).await;
        let user = insert_test_user(&core_context).await;
        let ip_address = fake_ipv4();

        let result = PostView::insert(&core_context, &post, Some(&user), &ip_address).await;

        assert!(result.is_ok());
    }
}
