use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use futures::future;

#[cfg(feature = "ssr")]
use mango3_core::pagination::CursorPage;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

mod action_form_resp;
mod basic_config_resp;
mod blob_resp;
mod post_resp;
mod user_profile_resp;
mod user_resp;
mod website_resp;

pub use action_form_resp::ActionFormResp;
pub use basic_config_resp::BasicConfigResp;
pub use blob_resp::BlobResp;
pub use post_resp::PostResp;
pub use user_profile_resp::UserProfileResp;
pub use user_resp::UserResp;
pub use website_resp::WebsiteResp;

#[cfg(feature = "ssr")]
#[async_trait]
pub trait FromCore<T> {
    async fn from_core(core_context: &CoreContext, value: &T) -> Self;
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CursorPageResp<R> {
    pub end_cursor: Option<String>,
    pub nodes: Vec<R>,
    pub has_next_page: bool,
}

impl<R> Default for CursorPageResp<R> {
    fn default() -> Self {
        Self {
            end_cursor: None,
            nodes: vec![],
            has_next_page: false,
        }
    }
}

#[cfg(feature = "ssr")]
#[async_trait]
impl<C, R> FromCore<CursorPage<C>> for CursorPageResp<R>
where
    C: Sync,
    R: FromCore<C> + Send,
{
    async fn from_core(core_context: &CoreContext, page: &CursorPage<C>) -> Self {
        Self {
            end_cursor: page.end_cursor.map(|c| c.to_string()),
            nodes: future::join_all(page.nodes.iter().map(|node| R::from_core(core_context, node))).await,
            has_next_page: page.has_next_page,
        }
    }
}
