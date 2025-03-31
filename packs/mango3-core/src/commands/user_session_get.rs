use std::future::Future;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use sqlx::query_as;
use uuid::Uuid;

use mango3_utils::models::UserSession;

use crate::constants::PREFIX_GET_USER_SESSION_BY_ID;
use crate::models::async_redis_cache;
use crate::CoreContext;

pub trait UserSessionGet {
    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = Result<UserSession, sqlx::Error>>;
}

impl UserSessionGet for UserSession {
    fn get_by_id(core_context: &CoreContext, id: Uuid) -> impl Future<Output = Result<UserSession, sqlx::Error>> {
        get_user_session_by_id(core_context, id)
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "AsyncRedisCache<Uuid, UserSession>",
    create = r##" { async_redis_cache(PREFIX_GET_USER_SESSION_BY_ID).await } "##
)]
pub(crate) async fn get_user_session_by_id(core_context: &CoreContext, id: Uuid) -> Result<UserSession, sqlx::Error> {
    query_as!(UserSession, "SELECT * FROM user_sessions WHERE id = $1 LIMIT 1", id)
        .fetch_one(&core_context.db_pool)
        .await
}
