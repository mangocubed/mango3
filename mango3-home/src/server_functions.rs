use leptos::prelude::*;

#[cfg(feature = "ssr")]
use futures::future;
#[cfg(feature = "ssr")]
use uuid::Uuid;

use mango3_leptos_utils::models::WebsiteResp;

#[cfg(feature = "ssr")]
use mango3_core::models::{PageParams, Website};

#[server]
pub async fn get_websites(after: Option<String>) -> Result<Vec<WebsiteResp>, ServerFnError> {
    use mango3_leptos_utils::ssr::expect_core_context;

    let core_context = expect_core_context();

    let page_params = PageParams {
        after: after.as_ref().and_then(|id| Uuid::try_parse(id).ok()),
        first: 10,
    };
    let websites = Website::paginate_by_created_at_desc(&core_context, &page_params, None, Some(true))
        .await
        .nodes;

    Ok(future::join_all(
        websites
            .iter()
            .map(|website| WebsiteResp::from_website(&core_context, website)),
    )
    .await)
}
