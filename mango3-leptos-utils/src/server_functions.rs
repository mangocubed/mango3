use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
use mango3_core::models::{Blob, Website};

use crate::models::{BlobResp, UserResp};

#[cfg(feature = "ssr")]
use crate::models::FromCore;
#[cfg(feature = "ssr")]
use crate::ssr::{expect_core_context, extract_user, require_authentication};

#[server(input = MultipartFormData)]
pub async fn attempt_to_upload_file(data: MultipartData) -> Result<Option<BlobResp>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    }

    let Some(mut data) = data.into_inner() else {
        return Ok(None);
    };

    let Some(field) = data.next_field().await? else {
        return Ok(None);
    };

    let Some("website_id") = field.name() else {
        return Ok(None);
    };

    let website_id = Uuid::try_parse(&field.text().await?).ok();

    let Some(mut field) = data.next_field().await? else {
        return Ok(None);
    };

    let Some("file") = field.name() else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let website = if let Some(website_id) = website_id {
        Website::get_by_id(&core_context, website_id, Some(&user), None)
            .await
            .ok()
    } else {
        None
    };

    let blob = Blob::insert(&core_context, &user, website.as_ref(), &mut field)
        .await
        .ok();

    Ok(blob.map(|blob| blob.into()))
}

#[server]
pub async fn get_current_user() -> Result<Option<UserResp>, ServerFnError> {
    let Some(user) = extract_user().await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(Some(UserResp::from_core(&core_context, &user).await))
}

#[server]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    crate::ssr::is_authenticated().await
}
