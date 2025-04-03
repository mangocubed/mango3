use crate::models::*;
use crate::utils::*;
use crate::CoreContext;

#[cfg(feature = "all-user-sessions-by-user")]
pub async fn all_user_sessions_by_user(core_context: &CoreContext, user: &crate::models::User) -> Vec<UserSession> {
    sqlx::query_as!(UserSession, "SELECT * FROM user_sessions WHERE user_id = $1", user.id)
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
}

#[cfg(feature = "delete-user-session")]
pub async fn delete_user_session(core_context: &CoreContext, user_session: &UserSession) -> crate::utils::MutResult {
    sqlx::query!("DELETE FROM user_sessions WHERE id = $1", user_session.id)
        .execute(&core_context.db_pool)
        .await?;

    use crate::utils::AsyncRedisCacheTrait;

    GET_USER_SESSION_BY_ID
        .cache_remove(crate::constants::PREFIX_GET_USER_SESSION_BY_ID, &user_session.id)
        .await;

    crate::mut_success!()
}

#[cfg(feature = "delete-all-user-sessions")]
pub async fn delete_all_user_sessions(
    core_context: &CoreContext,
    user: &crate::models::User,
) -> crate::utils::MutResult {
    futures::future::join_all(
        all_user_sessions_by_user(core_context, user)
            .await
            .iter()
            .map(|user_session| delete_user_session(core_context, user_session)),
    )
    .await;

    crate::mut_success!()
}

#[cfg(feature = "get-user-session-by-id")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "cached::AsyncRedisCache<uuid::Uuid, UserSession>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_USER_SESSION_BY_ID).await } "##
)]
pub async fn get_user_session_by_id(core_context: &CoreContext, id: uuid::Uuid) -> sqlx::Result<UserSession> {
    sqlx::query_as!(UserSession, "SELECT * FROM user_sessions WHERE id = $1 LIMIT 1", id)
        .fetch_one(&core_context.db_pool)
        .await
}

#[cfg(feature = "insert-user-session")]
pub async fn insert_user_session(core_context: &CoreContext, user: &User) -> MutResult<UserSession> {
    let result = sqlx::query_as!(
        UserSession,
        "INSERT INTO user_sessions (user_id) VALUES ($1) RETURNING *",
        user.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user_session) => {
            core_context
                .jobs
                .mailer(user, crate::enums::MailerJobCommand::NewUserSession)
                .await;

            crate::mut_success!(user_session)
        }
        Err(_) => crate::mut_error!(),
    }
}
