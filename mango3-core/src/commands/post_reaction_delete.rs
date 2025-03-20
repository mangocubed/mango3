use std::future::Future;

use sqlx::query;

use mango3_utils::models::PostReaction;

use crate::validator::ValidationErrors;
use crate::CoreContext;

pub trait PostReactionDelete {
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>>;
}

impl PostReactionDelete for PostReaction {
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>> {
        async {
            query!("DELETE FROM post_reactions WHERE id = $1", self.id)
                .execute(&core_context.db_pool)
                .await
                .map(|_| ())
                .map_err(|_| ValidationErrors::default())
        }
    }
}
