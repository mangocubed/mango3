use std::fmt::Display;

use cached::proc_macro::io_cached;
use cached::AsyncRedisCache;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::constants::KEY_TEXT_RESET_YOUR_PASSWORD;
use crate::models::{ConfirmationCode, User};
use crate::validator::ValidationErrors;
use crate::{async_redis_cache, CoreContext};

use super::AsyncRedisCacheTrait;

#[derive(Clone, Deserialize, Serialize)]
pub struct UserPasswordReset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub confirmation_code_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Display for UserPasswordReset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl UserPasswordReset {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM user_password_resets WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())?;

        GET_USER_PASSWORD_RESET_GET_BY_USER.cache_remove(&self.user_id).await;

        Ok(())
    }

    pub async fn get_by_user(core_context: &CoreContext, user: &User) -> sqlx::Result<Self> {
        get_user_password_reset_get_by_user(core_context, user).await
    }

    pub async fn confirmation_code(&self, core_context: &CoreContext) -> sqlx::Result<ConfirmationCode> {
        ConfirmationCode::get_by_id(core_context, self.confirmation_code_id).await
    }

    pub async fn delete_and_insert(core_context: &CoreContext, user: &User) -> Result<Self, ValidationErrors> {
        if let Ok(password_reset) = user.password_reset(core_context).await {
            let _ = password_reset.delete(core_context).await;
        };

        if !user.email_is_confirmed() {
            return Err(ValidationErrors::default());
        }

        let i18n = user.i18n();
        let action = i18n.text(KEY_TEXT_RESET_YOUR_PASSWORD);

        let confirmation_code = ConfirmationCode::insert(core_context, user, &action).await?;

        query_as!(
            Self,
            r#"INSERT INTO user_password_resets (user_id, confirmation_code_id) VALUES ($1, $2) RETURNING *"#,
            user.id,              // $1
            confirmation_code.id, // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ user.id }"#,
    ty = "AsyncRedisCache<Uuid, UserPasswordReset>",
    create = r##" { async_redis_cache("get_user_password_reset_get_by_user").await } "##
)]
pub async fn get_user_password_reset_get_by_user(
    core_context: &CoreContext,
    user: &User,
) -> sqlx::Result<UserPasswordReset> {
    query_as!(
        UserPasswordReset,
        "SELECT * FROM user_password_resets WHERE user_id = $1 LIMIT 1",
        user.id
    )
    .fetch_one(&core_context.db_pool)
    .await
}
