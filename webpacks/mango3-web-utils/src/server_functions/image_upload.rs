use leptos::prelude::*;

use server_fn::codec::{MultipartData, MultipartFormData};

use crate::presenters::{BlobPresenter, MutPresenter};

#[cfg(feature = "ssr")]
use crate::ssr::{expect_core_context, extract_user, require_authentication};

#[server(input = MultipartFormData)]
pub async fn attempt_to_upload_image(data: MultipartData) -> Result<MutPresenter<BlobPresenter>, ServerFnError> {
    if !require_authentication().await? {
        return crate::mut_presenter_error!();
    }

    let Some(mut data) = data.into_inner() else {
        return crate::mut_presenter_error!();
    };

    let Some(field) = data.next_field().await? else {
        return crate::mut_presenter_error!();
    };

    let Some("website_id") = field.name() else {
        return crate::mut_presenter_error!();
    };

    #[allow(unused_variables)]
    let website_id = field.text().await?;

    #[allow(unused_variables)]
    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    #[cfg(not(feature = "website-image-upload"))]
    let website = None;

    #[cfg(feature = "website-image-upload")]
    let website = Some(
        &mango3_core::commands::get_website_by_id(&core_context, uuid::Uuid::try_parse(&website_id)?, Some(&user))
            .await?,
    );

    let Some(mut field) = data.next_field().await? else {
        return crate::mut_presenter_error!();
    };

    let Some("file") = field.name() else {
        return crate::mut_presenter_error!();
    };

    let result = mango3_core::commands::insert_blob(&user, website, &mut field).await;

    crate::mut_presenter!(result)
}
