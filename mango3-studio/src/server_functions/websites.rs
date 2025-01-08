use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, CursorPageResp, WebsitePreviewResp, WebsiteResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, Website};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
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
    light_theme: String,
    dark_theme: String,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(website) = my_website(&id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let icon_image_blob = if let Some(id) = icon_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let result = website
        .update(
            &core_context,
            &name,
            &description,
            icon_image_blob.as_ref(),
            cover_image_blob.as_ref(),
            &light_theme,
            &dark_theme,
            publish.unwrap_or_default(),
        )
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_my_website(id: String) -> Result<Option<WebsiteResp>, ServerFnError> {
    if let Some(website) = my_website(&id).await? {
        let core_context = expect_core_context();

        Ok(Some(WebsiteResp::from_core(&core_context, &website).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_websites(after: Option<String>) -> Result<CursorPageResp<WebsitePreviewResp>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(CursorPageResp::default());
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Website::paginate_by_name_asc(&core_context, &page_params, Some(&user), None).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[cfg(feature = "ssr")]
pub async fn my_website(id: &str) -> Result<Option<Website>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(
        Website::get_by_id(&core_context, Uuid::try_parse(id)?, Some(&user), None)
            .await
            .ok(),
    )
}
