use leptos::prelude::*;

use mango3_leptos_utils::models::WebsiteResp;

#[cfg(feature = "ssr")]
use mango3_core::models::Website;
#[cfg(feature = "ssr")]
use mango3_leptos_utils::ssr::{expect_core_context, extract_host};

#[server]
pub async fn get_current_website() -> Result<Option<WebsiteResp>, ServerFnError> {
    let host = extract_host().await?;

    let Some(subdomain) = host.split(".").next() else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let result = Website::get_by_subdomain(&core_context, subdomain).await;

    if let Ok(website) = result {
        Ok(Some(WebsiteResp::from_website(&core_context, &website).await))
    } else {
        Ok(None)
    }
}
