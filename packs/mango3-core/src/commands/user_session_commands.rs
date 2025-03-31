use crate::models::{User, UserSession};
use crate::CoreContext;

#[cfg(feature = "all-user-sessions-by-user")]
pub async fn all_user_sessions_by_user(core_context: &CoreContext, user: &User) -> Vec<UserSession> {
    sqlx::query_as!(Self, "SELECT * FROM user_sessions WHERE user_id = $1", user.id)
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
}
