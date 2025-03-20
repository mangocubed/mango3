use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PostReaction {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub emoji: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
