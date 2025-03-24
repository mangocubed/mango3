use std::future::Future;

use sqlx::query_as;
use uuid::Uuid;

use mango3_utils::models::Hashtag;

use crate::CoreContext;

pub trait HashtagAll {
    fn all_by_ids(core_context: &CoreContext, ids: &Vec<Uuid>) -> impl Future<Output = Vec<Hashtag>>;
}

impl HashtagAll for Hashtag {
    fn all_by_ids(core_context: &CoreContext, ids: &Vec<Uuid>) -> impl Future<Output = Vec<Hashtag>> {
        async move {
            if ids.is_empty() {
                return vec![];
            }

            query_as!(
                Self,
                "SELECT * FROM hashtags WHERE id = ANY($1)",
                ids // $1
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::setup_core_context;

    use super::{Hashtag, HashtagAll};

    #[tokio::test]
    async fn should_return_all_by_ids() {
        let core_context = setup_core_context().await;

        let result = Hashtag::all_by_ids(&core_context, &vec![]).await;

        assert!(result.is_empty());
    }
}
