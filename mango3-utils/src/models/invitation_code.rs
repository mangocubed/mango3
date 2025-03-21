use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct InvitationCode {
    pub id: Uuid,
    pub email: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
