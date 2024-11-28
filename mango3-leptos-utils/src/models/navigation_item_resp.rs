use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use mango3_core::models::NavigationItem;

#[derive(Clone, Deserialize, Serialize)]
pub struct NavigationItemResp {
    pub id: String,
    pub position: i16,
    pub title: String,
    pub url: String,
}

#[cfg(feature = "ssr")]
impl From<&NavigationItem> for NavigationItemResp {
    fn from(navigation_item: &NavigationItem) -> Self {
        Self {
            id: navigation_item.id.to_string(),
            position: navigation_item.position,
            title: navigation_item.title.clone(),
            url: navigation_item.url.clone(),
        }
    }
}
