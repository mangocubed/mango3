use leptos::prelude::*;

use crate::models::BasicConfigResp;

#[server]
pub async fn get_basic_config() -> Result<BasicConfigResp, ServerFnError> {
    Ok(mango3_core::config::BASIC_CONFIG.clone().into())
}
