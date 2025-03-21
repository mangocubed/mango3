use sqlx::query_as;

use mango3_utils::models::UserSession;

use crate::models::User;
use crate::CoreContext;

pub(crate) trait UserSessionAll {
    async fn all_by_user(core_context: &CoreContext, user: &User) -> Vec<UserSession>;
}

impl UserSessionAll for UserSession {
    async fn all_by_user(core_context: &CoreContext, user: &User) -> Vec<UserSession> {
        query_as!(Self, "SELECT * FROM user_sessions WHERE user_id = $1", user.id)
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
    }
}
