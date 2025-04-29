#[cfg(feature = "with-dioxus")]
mod app_config_presenter;
#[cfg(feature = "with-dioxus")]
mod app_info_presenter;
#[cfg(feature = "with-dioxus")]
mod app_routes_presenter;
#[cfg(not(feature = "with-dioxus"))]
mod basic_config_presenter;
#[cfg(feature = "blob-presenter")]
mod blob_presenter;
#[cfg(feature = "cursor-page-presenter")]
mod cursor_page_presenter;
#[cfg(feature = "hashtag-presenter")]
mod hashtag_presenter;
#[cfg(not(feature = "with-dioxus"))]
mod info_presenter;
#[cfg(feature = "mutation-presenter")]
mod mutation_presenter;
#[cfg(feature = "navigation-item-presenter")]
mod navigation_item_presenter;
#[cfg(feature = "post-comment-presenter")]
mod post_comment_presenter;
#[cfg(any(feature = "post-min-presenter", feature = "post-presenter"))]
mod post_presenter;
#[cfg(feature = "user-presenter")]
mod user_presenter;
#[cfg(any(feature = "website-min-presenter", feature = "website-presenter"))]
mod website_presenter;

#[cfg(feature = "with-dioxus")]
pub use app_config_presenter::AppConfigPresenter;
#[cfg(feature = "with-dioxus")]
pub use app_info_presenter::AppInfoPresenter;
#[cfg(feature = "with-dioxus")]
pub use app_routes_presenter::AppRoutesPresenter;
#[cfg(not(feature = "with-dioxus"))]
pub use basic_config_presenter::BasicConfigPresenter;
#[cfg(feature = "blob-presenter")]
pub use blob_presenter::BlobPresenter;
#[cfg(feature = "cursor-page-presenter")]
pub use cursor_page_presenter::CursorPagePresenter;
#[cfg(feature = "hashtag-presenter")]
pub use hashtag_presenter::HashtagPresenter;
#[cfg(not(feature = "with-dioxus"))]
pub use info_presenter::InfoPresenter;
#[cfg(feature = "mutation-presenter")]
pub use mutation_presenter::MutPresenter;
#[cfg(all(not(feature = "with-dioxus"), feature = "mutation-presenter"))]
pub use mutation_presenter::MutPresenterActionValue;
#[cfg(feature = "navigation-item-presenter")]
pub use navigation_item_presenter::NavigationItemPresenter;
#[cfg(feature = "post-comment-presenter")]
pub use post_comment_presenter::PostCommentPresenter;
#[cfg(feature = "post-min-presenter")]
pub use post_presenter::PostMinPresenter;
#[cfg(feature = "post-presenter")]
pub use post_presenter::PostPresenter;
#[cfg(feature = "user-presenter")]
pub use user_presenter::{UserMinPresenter, UserPresenter};
#[cfg(feature = "website-min-presenter")]
pub use website_presenter::WebsiteMinPresenter;
#[cfg(feature = "website-presenter")]
pub use website_presenter::WebsitePresenter;

#[cfg(feature = "ssr")]
pub trait FromModel<T> {
    fn from_model(model: &T) -> impl std::future::Future<Output = Self>;
}

#[cfg(feature = "ssr")]
impl FromModel<()> for () {
    async fn from_model(_: &()) -> Self {
        ()
    }
}

#[cfg(feature = "ssr")]
impl FromModel<bool> for bool {
    async fn from_model(value: &bool) -> Self {
        *value
    }
}

#[cfg(feature = "ssr")]
impl FromModel<uuid::Uuid> for uuid::Uuid {
    async fn from_model(value: &uuid::Uuid) -> Self {
        value.clone()
    }
}

#[cfg(all(feature = "ssr", feature = "confirmation-code-presenter"))]
impl FromModel<mango3_core::models::ConfirmationCode<'_>> for () {
    async fn from_model(_: &mango3_core::models::ConfirmationCode<'_>) -> Self {
        ()
    }
}

#[cfg(all(feature = "ssr", feature = "post-reaction-presenter"))]
impl FromModel<mango3_core::models::PostReaction<'_>> for () {
    async fn from_model(_: &mango3_core::models::PostReaction<'_>) -> Self {
        ()
    }
}

#[cfg(feature = "ssr")]
impl FromModel<mango3_core::models::UserSession> for () {
    async fn from_model(_: &mango3_core::models::UserSession) -> Self {
        ()
    }
}
