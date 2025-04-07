use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Hashtag {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
