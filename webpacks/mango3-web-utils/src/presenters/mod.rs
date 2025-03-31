mod basic_config_presenter;
mod info_presenter;

#[cfg(feature = "blob-presenter")]
mod blob_presenter;
#[cfg(feature = "cursor-page-presenter")]
mod cursor_page_presenter;
#[cfg(feature = "hashtag-presenter")]
mod hashtag_presenter;
#[cfg(feature = "post-comment-presenter")]
mod post_comment_presenter;
#[cfg(feature = "post-presenter")]
mod post_presenter;
#[cfg(feature = "user-presenter")]
mod user_presenter;
#[cfg(feature = "website-presenter")]
mod website_presenter;

pub use basic_config_presenter::BasicConfigPresenter;
pub use info_presenter::InfoPresenter;

#[cfg(feature = "blob-presenter")]
pub use blob_presenter::BlobPresenter;
#[cfg(feature = "cursor-page-presenter")]
pub use cursor_page_presenter::CursorPagePresenter;
#[cfg(feature = "hashtag-presenter")]
pub use hashtag_presenter::HashtagPresenter;
#[cfg(feature = "post-comment-presenter")]
pub use post_comment_presenter::PostCommentPresenter;
#[cfg(feature = "post-presenter")]
pub use post_presenter::{PostMinPresenter, PostPresenter};
#[cfg(feature = "user-presenter")]
pub use user_presenter::{UserMinPresenter, UserPresenter};
#[cfg(feature = "website-presenter")]
pub use website_presenter::{WebsiteMinPresenter, WebsitePresenter};

#[cfg(feature = "ssr")]
pub trait FromModel<T> {
    fn from_model(core_context: &mango3_core::CoreContext, model: &T) -> impl std::future::Future<Output = Self>;
}
