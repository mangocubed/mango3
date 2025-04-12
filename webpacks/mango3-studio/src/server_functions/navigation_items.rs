use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use mango3_web_utils::presenters::{MutPresenter, NavigationItemPresenter};

#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::extract_i18n;

#[cfg(feature = "ssr")]
use crate::server_functions::my_website;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavigationItemParam {
    pub id: String,
    pub title: String,
    pub url: String,
}

impl From<&NavigationItemPresenter> for NavigationItemParam {
    fn from(value: &NavigationItemPresenter) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title.clone(),
            url: value.url.clone(),
        }
    }
}

#[server]
pub async fn get_all_my_navigation_items(website_id: Uuid) -> Result<Vec<NavigationItemPresenter>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(vec![]);
    };

    Ok(futures::future::join_all(
        mango3_core::commands::all_navigation_items_by_website(&website)
            .await
            .iter()
            .map(|navigation_item| NavigationItemPresenter::from_model(navigation_item)),
    )
    .await)
}

#[server]
pub async fn attempt_to_save_navigation(
    website_id: Uuid,
    items: Option<Vec<NavigationItemParam>>,
) -> Result<MutPresenter, ServerFnError> {
    use crate::constants::ssr::{KEY_TEXT_FAILED_TO_SAVE_NAVIGATION, KEY_TEXT_NAVIGATION_SAVED_SUCCESSFULLY};

    let i18n = extract_i18n().await?;
    let error_message = i18n.text(KEY_TEXT_FAILED_TO_SAVE_NAVIGATION);

    let Some(website) = my_website(website_id).await? else {
        return mango3_web_utils::mut_presenter_error!(error_message);
    };

    let items = items
        .unwrap_or_default()
        .iter()
        .map(|item| (Uuid::try_parse(&item.id).ok(), item.title.clone(), item.url.clone()))
        .collect();

    let result = mango3_core::commands::insert_or_update_many_navigation_items(&website, items).await;

    let success_message = i18n.text(KEY_TEXT_NAVIGATION_SAVED_SUCCESSFULLY);

    mango3_web_utils::mut_presenter!(result, success_message, error_message)
}
