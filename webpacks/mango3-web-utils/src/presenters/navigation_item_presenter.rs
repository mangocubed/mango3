use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::NavigationItem;

#[cfg(feature = "ssr")]
use super::FromModel;

#[derive(Clone, Deserialize, Serialize)]
pub struct NavigationItemPresenter {
    pub id: Uuid,
    pub position: i16,
    pub title: String,
    pub url: String,
}

#[cfg(feature = "ssr")]
impl FromModel<NavigationItem<'_>> for NavigationItemPresenter {
    async fn from_model(navigation_item: &NavigationItem<'_>) -> Self {
        Self {
            id: navigation_item.id,
            position: navigation_item.position,
            title: navigation_item.title.to_string(),
            url: navigation_item.url.to_string(),
        }
    }
}
