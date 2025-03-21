use std::future::Future;

use sqlx::query;

use mango3_utils::models::InvitationCode;

use crate::validator::ValidationErrors;
use crate::CoreContext;

pub trait InvitationCodeDelete {
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>>;
}

impl InvitationCodeDelete for InvitationCode {
    fn delete(&self, core_context: &CoreContext) -> impl Future<Output = Result<(), ValidationErrors>> {
        async {
            query!("DELETE FROM invitation_codes WHERE id = $1", self.id)
                .execute(&core_context.db_pool)
                .await
                .map(|_| ())
                .map_err(|_| ValidationErrors::default())
        }
    }
}
