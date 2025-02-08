use std::fmt::Display;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::constants::PREFIX_GET_USER_SESSION_BY_ID;
use crate::enums::MailerJobCommand;
use crate::models::async_redis_cache;
use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::{AsyncRedisCacheTrait, User};

#[derive(Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for UserSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl UserSession {
    async fn all_by_user(core_context: &CoreContext, user: &User) -> Vec<UserSession> {
        query_as!(Self, "SELECT * FROM user_sessions WHERE user_id = $1", user.id)
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
    }

    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM user_sessions WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())?;

        GET_USER_SESSION_BY_ID
            .cache_remove(PREFIX_GET_USER_SESSION_BY_ID, &self.id)
            .await;

        Ok(())
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        future::join_all(
            Self::all_by_user(core_context, user)
                .await
                .iter()
                .map(|user_session| user_session.delete(core_context)),
        )
        .await;

        Ok(())
    }

    pub async fn exists(core_context: &CoreContext, id: Uuid) -> bool {
        query!("SELECT id FROM user_sessions WHERE id = $1 LIMIT 1", id)
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok()
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> Result<Self, sqlx::Error> {
        get_user_session_by_id(core_context, id).await
    }

    pub async fn insert(core_context: &CoreContext, user: &User) -> Result<Self, ValidationErrors> {
        let result = query_as!(
            Self,
            "INSERT INTO user_sessions (user_id) VALUES ($1) RETURNING *",
            user.id,
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user_session) => {
                core_context.jobs.mailer(user, MailerJobCommand::NewUserSession).await;

                Ok(user_session)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn user(&self, core_context: &CoreContext) -> Result<User, sqlx::Error> {
        User::get_by_id(core_context, self.user_id).await
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
