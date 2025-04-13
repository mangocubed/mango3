use std::borrow::Cow;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PostReaction<'a> {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub emoji: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
