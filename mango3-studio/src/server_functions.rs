use leptos::prelude::*;

#[cfg(feature = "ssr")]
use futures::future;
#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, WebsiteResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, PageParams, Website};
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, extract_user, require_authentication};

#[server]
pub async fn attempt_to_create_website(
    name: String,
    subdomain: String,
    description: String,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let result = Website::insert(&core_context, &user, &name, &subdomain, &description).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_website(
    id: String,
    name: String,
    description: String,
    icon_image_blob_id: Option<String>,
    cover_image_blob_id: Option<String>,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    if !require_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?;

    let icon_image_blob = if let Some(id) = icon_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, user.as_ref()).await.ok()
    } else {
        None
    };

    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, user.as_ref()).await.ok()
    } else {
        None
    };

    let website = Website::get_by_id(&core_context, Uuid::try_parse(&id)?, user.as_ref()).await?;
    let result = website
        .update(
            &core_context,
            &name,
            &description,
            publish.is_some(),
            icon_image_blob.as_ref(),
            cover_image_blob.as_ref(),
        )
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_my_website(id: String) -> Result<Option<WebsiteResp>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?;
    let result = Website::get_by_id(&core_context, Uuid::try_parse(&id)?, user.as_ref()).await;

    if let Ok(website) = result {
        Ok(Some(WebsiteResp::from_website(&core_context, &website).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_websites(after: Option<String>) -> Result<Vec<WebsiteResp>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(vec![]);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = PageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let websites = Website::paginate_by_name_asc(&core_context, &page_params, Some(&user))
        .await
        .nodes;

    Ok(future::join_all(
        websites
            .iter()
            .map(|website| WebsiteResp::from_website(&core_context, website)),
    )
    .await)
}
