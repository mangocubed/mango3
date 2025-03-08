use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{FormResp, NavigationItemResp};

#[cfg(feature = "ssr")]
use mango3_core::models::NavigationItem;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n};

#[cfg(feature = "ssr")]
use crate::server_functions::my_website;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavigationItemParam {
    id: String,
    title: String,
    url: String,
}

#[server]
pub async fn get_all_my_navigation_items(website_id: String) -> Result<Vec<NavigationItemResp>, ServerFnError> {
    let Some(website) = my_website(&website_id).await? else {
        return Ok(vec![]);
    };

    let core_context = expect_core_context();
    let items = NavigationItem::all_by_website(&core_context, &website).await;

    Ok(items.iter().map(|item| item.into()).collect())
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
