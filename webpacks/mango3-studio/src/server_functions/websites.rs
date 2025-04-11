use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::presenters::{CursorPagePresenter, MutPresenter, WebsiteMinPresenter, WebsitePresenter};

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_core::utils::CursorPageParams;
#[cfg(feature = "ssr")]
use mango3_web_utils::presenters::FromModel;
#[cfg(feature = "ssr")]
use mango3_web_utils::ssr::{expect_core_context, extract_i18n, extract_user, require_authentication};

#[server]
pub async fn attempt_to_create_website(
    name: String,
    subdomain: String,
    description: String,
) -> Result<MutPresenter, ServerFnError> {
    use crate::constants::ssr::{KEY_TEXT_FAILED_TO_CREATE_WEBSITE, KEY_TEXT_WEBSITE_CREATED_SUCCESSFULLY};

    let i18n = extract_i18n().await?;
    let error_message = i18n.text(KEY_TEXT_FAILED_TO_CREATE_WEBSITE);

    if !require_authentication().await? {
        return mango3_web_utils::mut_presenter_error!(error_message);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    if !user.can_insert_website(&core_context).await {
        return mango3_web_utils::mut_presenter_error!(error_message);
    }

    let result = mango3_core::commands::insert_website(&core_context, &user, &name, &subdomain, &description).await;
    let success_message = i18n.text(KEY_TEXT_WEBSITE_CREATED_SUCCESSFULLY);

    mango3_web_utils::mut_presenter!(result, success_message, error_message)
}

#[server]
pub async fn attempt_to_update_website(
    id: Uuid,
    name: String,
    description: String,
    icon_image_blob_id: Option<Uuid>,
    cover_image_blob_id: Option<Uuid>,
    light_theme: String,
    dark_theme: String,
    publish: Option<bool>,
) -> Result<MutPresenter, ServerFnError> {
    use crate::constants::ssr::{KEY_TEXT_FAILED_TO_UPDATE_WEBSITE, KEY_TEXT_WEBSITE_UPDATED_SUCCESSFULLY};

    let i18n = extract_i18n().await?;
    let error_message = i18n.text(KEY_TEXT_FAILED_TO_UPDATE_WEBSITE);

    let Some(website) = my_website(id).await? else {
        return mango3_web_utils::mut_presenter_error!(error_message);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let icon_image_blob = if let Some(id) = icon_image_blob_id {
        mango3_core::commands::get_blob_by_id(id, Some(&website), Some(&user))
            .await
            .ok()
    } else {
        None
    };

    let cover_image_blob = if let Some(id) = cover_image_blob_id {
        mango3_core::commands::get_blob_by_id(id, Some(&website), Some(&user))
            .await
            .ok()
    } else {
        None
    };

    let result = mango3_core::commands::update_website(
        &core_context,
        &website,
        &name,
        &description,
        icon_image_blob.as_ref(),
        cover_image_blob.as_ref(),
        &light_theme,
        &dark_theme,
        publish.unwrap_or_default(),
    )
    .await;
    let success_message = i18n.text(KEY_TEXT_WEBSITE_UPDATED_SUCCESSFULLY);

    mango3_web_utils::mut_presenter!(result, success_message, error_message)
}

#[server]
pub async fn get_my_website(id: String) -> Result<Option<WebsitePresenter>, ServerFnError> {
    if let Some(website) = my_website(Uuid::try_parse(&id)?).await? {
        Ok(Some(WebsitePresenter::from_model(&website).await))
    } else {
        Ok(None)
    }
}

#[server]
pub async fn get_my_websites(after: Option<Uuid>) -> Result<CursorPagePresenter<WebsiteMinPresenter>, ServerFnError> {
    if !require_authentication().await? {
        return mango3_web_utils::cursor_page_presenter!();
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();
    let page_params = CursorPageParams { after, first: 10 };
    let page =
        mango3_core::commands::paginate_websites_sorted_by_name_asc(&core_context, &page_params, Some(&user), None)
            .await;

    mango3_web_utils::cursor_page_presenter!(&page)
}

#[cfg(feature = "ssr")]
pub async fn my_website(id: Uuid) -> Result<Option<Website>, ServerFnError> {
    if !require_authentication().await? {
        return Ok(None);
    }

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    Ok(mango3_core::commands::get_website_by_id(&core_context, id, Some(&user))
        .await
        .ok())
}
