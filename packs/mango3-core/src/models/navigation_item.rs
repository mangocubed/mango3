use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct NavigationItem {
    pub id: Uuid,
    pub website_id: Uuid,
    pub position: i16,
    pub title: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct NavigationItems(Vec<NavigationItem>);

impl Display for NavigationItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|item| item.id.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl From<NavigationItems> for Vec<NavigationItem> {
    fn from(items: NavigationItems) -> Self {
        items.0
    }
}

impl From<Vec<NavigationItem>> for NavigationItems {
    fn from(items: Vec<NavigationItem>) -> Self {
        NavigationItems(items)
    }
}
