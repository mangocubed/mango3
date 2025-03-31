use leptos::prelude::*;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use futures::future;

use mango3_web_utils::models::{BlobResp, FormResp};
use mango3_utils::models::CursorPage;

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, User, Website};
#[cfg(feature = "ssr")]
use mango3_web_utils::models::FromCore;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n, extract_user};
#[cfg(feature = "ssr")]
use mango3_utils::models::CursorPageParams;

#[cfg(feature = "ssr")]
use super::my_website;

#[server]
pub async fn attempt_to_delete_blob(website_id: String, id: String) -> Result<FormResp, ServerFnError> {
    let i18n = extract_i18n().await?;

    let Some(blob) = my_blob(&website_id, &id).await? else {
        return FormResp::new_with_error(&i18n);
    };

    let core_context = expect_core_context();

    let result = blob.delete(&core_context).await;

    FormResp::new(&i18n, result)
}

#[cfg(feature = "ssr")]
pub async fn get_blobs_by_ids(website: &Website, user: &User, ids: Option<Vec<String>>) -> Vec<Blob> {
    let Some(ids) = ids else {
        return vec![];
    };

    let core_context = expect_core_context();

    Blob::all_by_ids(
        &core_context,
        ids.iter().map(|id| Uuid::try_parse(id).unwrap()).collect(),
        Some(&website),
        Some(&user),
    )
    .await
}

#[server]
pub async fn get_my_blobs(website_id: String, after: Option<Uuid>) -> Result<CursorPage<BlobResp>, ServerFnError> {
    let Some(website) = my_website(&website_id).await? else {
        return Ok(CursorPage::default());
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams { after, first: 10 };
    let page = Blob::paginate_by_created_at_desc(&core_context, &page_params, Some(&website), Some(&user)).await;

    Ok(CursorPage {
        end_cursor: page.end_cursor,
        has_next_page: page.has_next_page,
        nodes: future::join_all(page.nodes.iter().map(|blob| BlobResp::from_core(&core_context, blob))).await,
    })
}

#[cfg(feature = "ssr")]
async fn my_blob(website_id: &str, id: &str) -> Result<Option<Blob>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(
        Blob::get_by_id(&core_context, Uuid::try_parse(id)?, Some(&website), Some(&user))
            .await
            .ok(),
    )
}
