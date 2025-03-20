use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use uuid::Uuid;

pub struct PostView {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Option<Uuid>,
    pub ip_address: IpNetwork,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
