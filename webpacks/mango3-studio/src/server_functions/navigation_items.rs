use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_web_utils::models::FormResp;
use mango3_utils::models::NavigationItem;

#[cfg(feature = "ssr")]
use mango3_core::commands::{NavigationItemAll, NavigationItemBulkWrite};
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n};

#[cfg(feature = "ssr")]
use crate::server_functions::my_website;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavigationItemParam {
    pub id: String,
    pub position: i16,
    pub title: String,
    pub url: String,
}

impl From<&NavigationItem> for NavigationItemParam {
    fn from(value: &NavigationItem) -> Self {
        Self {
            id: value.id.to_string(),
            position: value.position,
            title: value.title.clone(),
            url: value.url.clone(),
        }
    }
}

#[server]
pub async fn get_all_my_navigation_items(website_id: String) -> Result<Vec<NavigationItem>, ServerFnError> {
    let Some(website) = my_website(&website_id).await? else {
        return Ok(vec![]);
    };

    let core_context = expect_core_context();

    Ok(NavigationItem::all_by_website(&core_context, &website).await)
}

#[server]
pub async fn attempt_to_save_navigation(
    website_id: String,
    items: Option<Vec<NavigationItemParam>>,
) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(website) = my_website(&website_id).await? else {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let items = items
        .unwrap_or_default()
        .iter()
        .map(|item| (Uuid::try_parse(&item.id).ok(), item.title.clone(), item.url.clone()))
        .collect();

    let result = NavigationItem::save_all(&core_context, &website, items).await;

    FormResp::new(&i18n, result)
}
