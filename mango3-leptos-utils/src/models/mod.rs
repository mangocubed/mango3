#[cfg(feature = "ssr")]
use async_trait::async_trait;

#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

mod basic_config_resp;
mod blob_resp;
mod info_resp;
mod post_comment_resp;
mod post_resp;
mod user_profile_resp;
mod user_resp;
mod website_resp;

#[cfg(feature = "forms")]
mod form_resp;

pub use basic_config_resp::BasicConfigResp;
pub use blob_resp::BlobResp;
pub use info_resp::InfoResp;
pub use post_comment_resp::PostCommentResp;
pub use post_resp::{PostPreviewResp, PostResp};
pub use user_profile_resp::UserProfileResp;
pub use user_resp::{UserPreviewResp, UserResp};
pub use website_resp::{WebsitePreviewResp, WebsiteResp};

#[cfg(feature = "forms")]
pub use form_resp::{ActionValue, FormResp};

#[cfg(feature = "ssr")]
#[async_trait]
pub trait FromCore<T> {
    async fn from_core(core_context: &CoreContext, value: &T) -> Self;
}
