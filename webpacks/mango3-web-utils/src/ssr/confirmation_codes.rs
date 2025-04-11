use leptos::prelude::*;
use uuid::Uuid;

use mango3_core::commands::get_confirmation_code_by_id;
use mango3_core::models::ConfirmationCode;

use crate::constants::KEY_CONFIRMATION_CODE_ID;

use super::extract_session;

pub async fn extract_confirmation_code<'a>() -> Result<Option<ConfirmationCode<'a>>, ServerFnError> {
    let session = extract_session().await?;

    let Some(id) = session.get(KEY_CONFIRMATION_CODE_ID).await? else {
        return Ok(None);
    };

    Ok(get_confirmation_code_by_id(id).await.ok())
}

pub async fn finish_confirmation_code() -> Result<(), ServerFnError> {
    let session = extract_session().await?;

    session.remove::<Uuid>(KEY_CONFIRMATION_CODE_ID).await?;

    Ok(())
}

pub async fn start_confirmation_code(confirmation_code: &ConfirmationCode<'_>) -> Result<(), ServerFnError> {
    let session = extract_session().await?;

    session.insert(KEY_CONFIRMATION_CODE_ID, confirmation_code.id).await?;

    Ok(())
}
