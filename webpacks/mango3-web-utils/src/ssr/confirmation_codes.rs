use leptos::prelude::*;
use uuid::Uuid;

use mango3_core::models::ConfirmationCode;

use crate::constants::KEY_CONFIRMATION_CODE_ID;

use super::{expect_core_context, extract_session};

pub async fn extract_confirmation_code() -> Result<Option<ConfirmationCode>, ServerFnError> {
    let session = extract_session().await?;

    let Some(id) = session.get(KEY_CONFIRMATION_CODE_ID).await? else {
        return Ok(None);
    };

    let core_context = expect_core_context();

    Ok(ConfirmationCode::get_by_id(&core_context, id).await.ok())
}

pub async fn finish_confirmation_code() -> Result<(), ServerFnError> {
    let session = extract_session().await?;

    session.remove::<Uuid>(KEY_CONFIRMATION_CODE_ID).await?;

    Ok(())
}

pub async fn start_confirmation_code(confirmation_code: &ConfirmationCode) -> Result<(), ServerFnError> {
    let session = extract_session().await?;

    session.insert(KEY_CONFIRMATION_CODE_ID, confirmation_code.id).await?;

    Ok(())
}
