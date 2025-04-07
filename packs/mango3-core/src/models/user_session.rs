use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::commands::get_user_by_id;
use crate::CoreContext;

use super::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub async fn user(&self, core_context: &CoreContext) -> sqlx::Result<User> {
        get_user_by_id(core_context, self.user_id).await
    }
}
