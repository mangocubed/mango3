use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct NavigationItem<'a> {
    pub id: Uuid,
    pub website_id: Uuid,
    pub position: i16,
    pub title: Cow<'a, str>,
    pub url: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct NavigationItems<'a>(Vec<NavigationItem<'a>>);

impl Display for NavigationItems<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|item| item.id.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<'a> From<NavigationItems<'a>> for Vec<NavigationItem<'a>> {
    fn from(items: NavigationItems<'a>) -> Self {
        items.0
    }
}

impl<'a> From<Vec<NavigationItem<'a>>> for NavigationItems<'a> {
    fn from(items: Vec<NavigationItem<'a>>) -> Self {
        NavigationItems(items)
    }
}
