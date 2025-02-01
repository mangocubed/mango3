use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::{query, query_as};

use crate::validator::ValidationErrors;
use crate::CoreContext;

use super::User;

pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserSession {
    pub async fn delete(&self, core_context: &CoreContext) -> Result<(), ValidationErrors> {
        query!("DELETE FROM user_sessions WHERE id = $1", self.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn delete_all(core_context: &CoreContext, user: &User) -> Result<(), ValidationErrors> {
        query!("DELETE FROM user_sessions WHERE user_id = $1", user.id)
            .execute(&core_context.db_pool)
            .await
            .map(|_| ())
            .map_err(|_| ValidationErrors::default())
    }

    pub async fn exists(core_context: &CoreContext, id: Uuid) -> bool {
        query!("SELECT id FROM user_sessions WHERE id = $1 LIMIT 1", id)
            .fetch_one(&core_context.db_pool)
            .await
            .is_ok()
    }

    pub async fn get_by_id(core_context: &CoreContext, id: Uuid) -> Result<Self, sqlx::Error> {
        query_as!(Self, "SELECT * FROM user_sessions WHERE id = $1 LIMIT 1", id)
            .fetch_one(&core_context.db_pool)
            .await
    }

    pub async fn insert(core_context: &CoreContext, user: &User) -> Result<Self, ValidationErrors> {
        query_as!(
            Self,
            "INSERT INTO user_sessions (user_id) VALUES ($1) RETURNING *",
            user.id,
        )
        .fetch_one(&core_context.db_pool)
        .await
        .map_err(|_| ValidationErrors::default())
    }

    pub async fn user(&self, core_context: &CoreContext) -> Result<User, sqlx::Error> {
        User::get_by_id(core_context, self.user_id).await
    }
}
