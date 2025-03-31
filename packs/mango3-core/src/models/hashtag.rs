use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Hashtag {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
