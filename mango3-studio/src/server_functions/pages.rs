use leptos::prelude::*;

#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::{ActionFormResp, CursorPageResp, PageResp};

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, Page};
#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, extract_user};

#[cfg(feature = "ssr")]
use super::my_website;

#[server]
pub async fn attempt_to_create_page(
    website_id: String,
    title: String,
    slug: String,
    content: String,
    cover_image_blob_id: Option<String>,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(website) = my_website(&website_id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let result = Page::insert(
        &core_context,
        &website,
        &user,
        &title,
        &slug,
        &content,
        cover_image_blob.as_ref(),
        publish.unwrap_or_default(),
    )
    .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_delete_page(website_id: String, id: String) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(page) = my_page(&website_id, &id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let result = page.delete(&core_context).await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn attempt_to_update_page(
    website_id: String,
    id: String,
    title: String,
    slug: String,
    content: String,
    cover_image_blob_id: Option<String>,
    publish: Option<bool>,
) -> Result<ActionFormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(page) = my_page(&website_id, &id).await? else {
        return ActionFormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let cover_image_blob = if let Some(id) = cover_image_blob_id.as_ref().and_then(|id| Uuid::try_parse(id).ok()) {
        Blob::get_by_id(&core_context, id, Some(&user)).await.ok()
    } else {
        None
    };

    let result = page
        .update(
            &core_context,
            &title,
            &slug,
            &content,
            cover_image_blob.as_ref(),
            publish.unwrap_or_default(),
        )
        .await;

    ActionFormResp::new(&i18n, result)
}

#[server]
pub async fn get_my_page(website_id: String, id: String) -> Result<Option<PageResp>, ServerFnError> {
    if let Some(page) = my_page(&website_id, &id).await? {
        let core_context = expect_core_context();

        Ok(Some(PageResp::from_core(&core_context, &page).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_pages(
    website_id: String,
    after: Option<String>,
) -> Result<CursorPageResp<PageResp>, ServerFnError> {
    let Some(website) = my_website(&website_id).await? else {
        return Ok(CursorPageResp::default());
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let page = Page::paginate_by_created_at_desc(&core_context, &page_params, Some(&website), Some(&user), None).await;

    Ok(CursorPageResp::from_core(&core_context, &page).await)
}

#[cfg(feature = "ssr")]
async fn my_page(website_id: &str, id: &str) -> Result<Option<Page>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(
        Page::get_by_id(&core_context, Uuid::try_parse(id)?, Some(&website), Some(&user))
            .await
            .ok(),
    )
}
