use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use crate::models::{BlobResp, UserResp};

#[cfg(feature = "ssr")]
use crate::models::FromCore;
#[cfg(feature = "ssr")]
use crate::ssr::{expect_core_context, extract_user};

#[server(input = MultipartFormData)]
pub async fn attempt_to_upload_file(data: MultipartData) -> Result<Option<BlobResp>, ServerFnError> {
    use crate::ssr::{expect_core_context, extract_user, require_authentication};

    use mango3_core::models::Blob;

    if !require_authentication().await? {
        return Ok(None);
    }

    let Some(mut data) = data.into_inner() else {
        return Ok(None);
    };

    let Some(mut field) = data.next_field().await? else {
        return Ok(None);
    };

    let Some("file") = field.name() else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let blob = Blob::insert(&core_context, &user, &mut field).await.ok();

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
