use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{BlobPresenter, CursorPagePresenter, MutPresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, User, Website};
#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_user};

#[cfg(feature = "ssr")]
use super::my_website;

#[server]
pub async fn attempt_to_delete_blob(website_id: Uuid, id: Uuid) -> Result<MutPresenter, ServerFnError> {
    let Some(blob) = my_blob(website_id, id).await? else {
        return mango3_web_utils::mut_presenter_error!();
    };

    let core_context = expect_core_context();

    let result = mango3_core::commands::delete_blob(&core_context, &blob).await;

    mango3_web_utils::mut_presenter!(result)
}

#[cfg(feature = "ssr")]
pub async fn get_blobs_by_ids(website: &Website, user: &User, ids: Option<Vec<Uuid>>) -> Vec<Blob> {
    let Some(ids) = ids else {
        return vec![];
    };

    let core_context = expect_core_context();

    mango3_core::commands::all_blobs_by_ids(&core_context, ids, Some(&website), Some(&user)).await
}

#[server]
pub async fn get_my_blobs(
    website_id: Uuid,
    after: Option<Uuid>,
) -> Result<CursorPagePresenter<BlobPresenter>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return mango3_web_utils::cursor_page_presenter!();
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams { after, first: 10 };
    let page = mango3_core::commands::paginate_blobs(&core_context, &page_params, Some(&website), Some(&user)).await;

    mango3_web_utils::cursor_page_presenter!(&page)
}

#[cfg(feature = "ssr")]
async fn my_blob(website_id: Uuid, id: Uuid) -> Result<Option<Blob>, ServerFnError> {
    let Some(website) = my_website(website_id).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(
        mango3_core::commands::get_blob_by_id(&core_context, id, Some(&website), Some(&user))
            .await
            .ok(),
    )
}
