use std::fmt::Display;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use futures::future;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::constants::{KEY_TEXT_CONFIRM_YOUR_LOGIN, PREFIX_GET_USER_SESSION_BY_ID};
use crate::enums::{Input, InputError, MailerJobCommand};
use crate::models::async_redis_cache;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::{AsyncRedisCacheTrait, ConfirmationCode, User};

#[derive(Clone, Deserialize, Serialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub confirmation_code_id: Option<Uuid>,
    pub confirmed_at: Option<DateTime<Utc>>,
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

    pub async fn confirm(&self, core_context: &CoreContext, code: &str) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        if validator.validate_presence(Input::Code, code) {
            let confirmation_code = self
                .confirmation_code(core_context)
                .await
                .ok_or_else(ValidationErrors::default)?
                .map_err(|_| ValidationErrors::default())?;
            let code_is_verified = confirmation_code.verify_code(core_context, code).await;

            validator.custom_validation(Input::Code, InputError::IsInvalid, &|| code_is_verified);
        }

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let result = query_as!(
            Self,
            r#"UPDATE user_sessions SET confirmation_code_id = NULL, confirmed_at = current_timestamp
            WHERE confirmed_at IS NULL AND id = $1 RETURNING *"#,
            self.id, // $1
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user_session) => {
                if let Ok(user) = user_session.user(core_context).await {
                    core_context.jobs.mailer(&user, MailerJobCommand::NewUserSession).await;
                }

                Ok(user_session)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub async fn confirmation_code(&self, core_context: &CoreContext) -> Option<sqlx::Result<ConfirmationCode>> {
        if let Some(confirmation_code_id) = self.confirmation_code_id {
            Some(ConfirmationCode::get_by_id(core_context, confirmation_code_id).await)
        } else {
            None
        }
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

    pub async fn delete_all_expired(core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM user_sessions WHERE confirmed_at IS NULL AND created_at < current_timestamp - INTERVAL '1 hour'")
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn get_by_confirmation_code(
        core_context: &CoreContext,
        confirmation_code: &ConfirmationCode,
    ) -> sqlx::Result<Self> {
        query_as!(
            Self,
            "SELECT * FROM user_sessions WHERE confirmed_at IS NULL AND confirmation_code_id = $1 LIMIT 1",
            confirmation_code.id
        )
        .fetch_one(&core_context.db_pool)
        .await
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> Result<Self, sqlx::Error> {
        get_user_session_by_id(core_context, id).await
    }

    pub async fn insert(
        core_context: &CoreContext,
        user: &User,
        force_confirmation: bool,
    ) -> Result<Self, ValidationErrors> {
        let confirmation_code_id = if !force_confirmation && user.email_is_confirmed() {
            let action = user.i18n().text(KEY_TEXT_CONFIRM_YOUR_LOGIN);

            Some(ConfirmationCode::insert(core_context, user, &action).await?.id)
        } else {
            None
        };

        let result = query_as!(
            Self,
            "INSERT INTO user_sessions (user_id, confirmation_code_id, confirmed_at)
            VALUES ($1, $2, CASE WHEN $2::uuid IS NULL THEN current_timestamp ELSE NULL END)
            RETURNING *",
            user.id,              // $1
            confirmation_code_id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user_session) => {
                if user_session.is_confirmed() {
                    core_context.jobs.mailer(user, MailerJobCommand::NewUserSession).await;
                }

                Ok(user_session)
            }
            Err(_) => Err(ValidationErrors::default()),
        }
    }

    pub fn is_confirmed(&self) -> bool {
        self.confirmed_at.is_some()
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
    query_as!(
        UserSession,
        "SELECT * FROM user_sessions WHERE confirmed_at IS NOT NULL AND id = $1 LIMIT 1",
        id
    )
    .fetch_one(&core_context.db_pool)
    .await
}
