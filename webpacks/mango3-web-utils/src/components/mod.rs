#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos_meta::{Link, Title};

use crate::context::use_basic_config;

#[cfg(not(feature = "with-dioxus"))]
use crate::i18n::{t, use_i18n};
#[cfg(feature = "with-dioxus")]
use crate::prelude::*;

mod app_provider;

#[cfg(not(feature = "with-dioxus"))]
mod alert_modal;
#[cfg(not(feature = "with-dioxus"))]
mod authentication;
#[cfg(not(feature = "with-dioxus"))]
mod bottom_bar;
#[cfg(not(feature = "with-dioxus"))]
mod brand;
#[cfg(not(feature = "with-dioxus"))]
mod confirmation_modal;
#[cfg(not(feature = "with-dioxus"))]
mod copyable_text;
#[cfg(not(feature = "with-dioxus"))]
mod hashtags;
#[cfg(not(feature = "with-dioxus"))]
mod loading_overlay;
#[cfg(not(feature = "with-dioxus"))]
mod loading_spinner;
#[cfg(not(feature = "with-dioxus"))]
mod menu;
#[cfg(not(feature = "with-dioxus"))]
mod modal;
#[cfg(not(feature = "with-dioxus"))]
mod post_bottom_bar;
#[cfg(not(feature = "with-dioxus"))]
mod search_bar;
#[cfg(not(feature = "with-dioxus"))]
mod time_ago;
#[cfg(not(feature = "with-dioxus"))]
mod top_bar;

#[cfg(all(not(feature = "with-dioxus"), feature = "current-user"))]
mod current_user;
#[cfg(all(not(feature = "with-dioxus"), feature = "infinite-scroll"))]
mod infinite_scroll;
#[cfg(all(not(feature = "with-dioxus"), feature = "post-card"))]
mod post_card;
#[cfg(all(not(feature = "with-dioxus"), feature = "unconfirmed-email-alert"))]
mod unconfirmed_email_alert;
#[cfg(all(not(feature = "with-dioxus"), feature = "user-card"))]
mod user_card;
#[cfg(all(not(feature = "with-dioxus"), feature = "user-tag"))]
mod user_tag;
#[cfg(all(not(feature = "with-dioxus"), feature = "website-card"))]
mod website_card;

#[cfg(all(not(feature = "with-dioxus"), feature = "forms"))]
pub mod forms;

pub use app_provider::AppProvider;

#[cfg(not(feature = "with-dioxus"))]
pub use alert_modal::AlertModal;
#[cfg(not(feature = "with-dioxus"))]
pub use authentication::{RequireAuthentication, RequireNoAuthentication};
#[cfg(not(feature = "with-dioxus"))]
pub use bottom_bar::BottomBar;
#[cfg(not(feature = "with-dioxus"))]
pub use brand::Brand;
#[cfg(not(feature = "with-dioxus"))]
pub use confirmation_modal::ConfirmationModal;
#[cfg(not(feature = "with-dioxus"))]
pub use copyable_text::CopyableText;
#[cfg(not(feature = "with-dioxus"))]
pub use hashtags::Hashtags;
#[cfg(not(feature = "with-dioxus"))]
pub use loading_overlay::LoadingOverlay;
#[cfg(not(feature = "with-dioxus"))]
pub use loading_spinner::LoadingSpinner;
#[cfg(not(feature = "with-dioxus"))]
pub use menu::{Menu, MenuItem};
#[cfg(not(feature = "with-dioxus"))]
pub use modal::Modal;
#[cfg(not(feature = "with-dioxus"))]
pub use post_bottom_bar::PostBottomBar;
#[cfg(not(feature = "with-dioxus"))]
pub use search_bar::SearchBar;
#[cfg(not(feature = "with-dioxus"))]
pub use time_ago::TimeAgo;
#[cfg(not(feature = "with-dioxus"))]
pub use top_bar::TopBar;

#[cfg(all(feature = "current-user", not(feature = "with-dioxus")))]
pub use current_user::{CurrentUser, CurrentUserOpt};
#[cfg(all(feature = "infinite-scroll", not(feature = "with-dioxus")))]
pub use infinite_scroll::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController,
    InfiniteScrollResourceController,
};
#[cfg(all(feature = "post-card", not(feature = "with-dioxus")))]
pub use post_card::PostCard;
#[cfg(all(feature = "unconfirmed-email-alert", not(feature = "with-dioxus")))]
pub use unconfirmed_email_alert::UnconfirmedEmailAlert;
#[cfg(all(feature = "user-card", not(feature = "with-dioxus")))]
pub use user_card::UserCard;
#[cfg(all(feature = "user-tag", not(feature = "with-dioxus")))]
pub use user_tag::{UserAvatar, UserLabels, UserTag, UserTagLink};
#[cfg(all(feature = "website-card", not(feature = "with-dioxus")))]
pub use website_card::WebsiteCard;

#[cfg(not(feature = "with-dioxus"))]
pub struct BoxedFn(pub Box<dyn Fn() + 'static>);

#[cfg(not(feature = "with-dioxus"))]
impl<T> From<T> for BoxedFn
where
    T: Fn() + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn AppTitle(#[prop(optional, into)] suffix: Signal<Option<String>>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <Title formatter=move |page_title: String| {
            let mut text = (if page_title.is_empty() { String::new() } else { format!("{page_title} | ") })
                + &basic_config.title;
            if let Some(suffix) = &suffix.get() { text + &format!(" {suffix}") } else { text }
        } />
    }
}

#[cfg(feature = "with-dioxus")]
#[component]
pub fn FaviconLink(href: Option<String>) -> Element {
    let basic_config = use_basic_config()?;

    let href = if let Some(href) = href {
        href
    } else {
        basic_config.asset_url("favicon.ico").to_string()
    };

    rsx! { document::Link { rel: "icon",  href: href } }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn FaviconLink(#[prop(into, optional)] href: Option<String>) -> impl IntoView {
    let basic_config = use_basic_config();

    let href = if let Some(href) = href {
        href
    } else {
        basic_config.asset_url("favicon.ico").to_string()
    };

    view! { <Link rel="icon" href=href /> }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn GoToMango3() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();

    view! {
        <a class="btn btn-ghost btn-block px-2 font-normal" href=basic_config.home_url.to_string()>
            {t!(
                i18n,
                shared.go_to_title,
                title = {
                    let icon_url = basic_config.asset_url("icon.svg");
                    let title = basic_config.title.clone();
                    move || view! {
                        <img alt=title.clone() class="h-[16px]" src=icon_url.to_string() />
                        <span class="font-bold">{title.clone()}</span>
                    }
                }
            )}
        </a>
    }
}

#[cfg(all(not(feature = "with-dioxus"), feature = "website-icon"))]
#[component]
pub fn WebsiteIcon(
    #[prop(into, optional)] class: &'static str,
    #[prop(into)] website: crate::presenters::WebsiteMinPresenter,
    #[prop(default = 32)] size: u16,
) -> impl IntoView {
    view! {
        <div class=format!("avatar {class}")>
            <div class="rounded" style:width=format!("{size}px") style:height=format!("{size}px")>
                <img alt=website.initials.clone() src=website.icon_image_url(size).to_string() />
            </div>
        </div>
    }
}
