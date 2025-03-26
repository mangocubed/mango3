use leptos::prelude::*;
use leptos_meta::{Link, Title};

use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};
use crate::models::WebsitePreviewResp;

mod alert_modal;
mod app_provider;
mod authentication;
mod bottom_bar;
mod brand;
mod confirmation_modal;
mod copyable_text;
mod current_user;
mod hashtags;
mod loading_overlay;
mod loading_spinner;
mod menu;
mod modal;
mod post_bottom_bar;
mod search_bar;
mod time_ago;
mod top_bar;
mod unconfirmed_email_alert;
mod user_tag;

#[cfg(feature = "infinite_scroll")]
mod infinite_scroll;
#[cfg(feature = "post_card")]
mod post_card;
#[cfg(feature = "user_card")]
mod user_card;
#[cfg(feature = "website_card")]
mod website_card;

#[cfg(feature = "forms")]
pub mod forms;

pub use alert_modal::AlertModal;
pub use app_provider::AppProvider;
pub use authentication::{RequireAuthentication, RequireNoAuthentication};
pub use bottom_bar::BottomBar;
pub use brand::Brand;
pub use confirmation_modal::ConfirmationModal;
pub use copyable_text::CopyableText;
pub use current_user::{CurrentUser, CurrentUserOpt};
pub use hashtags::Hashtags;
pub use loading_overlay::LoadingOverlay;
pub use loading_spinner::LoadingSpinner;
pub use menu::{Menu, MenuItem};
pub use modal::Modal;
pub use post_bottom_bar::PostBottomBar;
pub use search_bar::SearchBar;
pub use time_ago::TimeAgo;
pub use top_bar::TopBar;
pub use unconfirmed_email_alert::UnconfirmedEmailAlert;
pub use user_tag::{UserAvatar, UserLabels, UserTag, UserTagLink};

#[cfg(feature = "infinite_scroll")]
pub use infinite_scroll::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController,
    InfiniteScrollResourceController,
};
#[cfg(feature = "post_card")]
pub use post_card::PostCard;
#[cfg(feature = "user_card")]
pub use user_card::UserCard;
#[cfg(feature = "website_card")]
pub use website_card::WebsiteCard;

pub struct BoxedFn(pub Box<dyn Fn() + 'static>);

impl<T> From<T> for BoxedFn
where
    T: Fn() + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

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

#[component]
pub fn FaviconLink(#[prop(into, optional)] href: Option<String>) -> impl IntoView {
    let basic_config = use_basic_config();

    let href = if let Some(href) = href {
        href
    } else {
        basic_config.asset_url("favicon.ico")
    };

    view! { <Link rel="icon" href=href /> }
}

#[component]
pub fn GoToMango3() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();

    view! {
        <a class="btn btn-ghost btn-block px-2 font-normal" href=basic_config.home_url.clone()>
            {t!(
                i18n,
                shared.go_to_title,
                title = {
                    let icon_url = basic_config.asset_url("icon.svg");
                    let title = basic_config.title.clone();
                    move || view! {
                        <img alt=title.clone() class="h-[16px]" src=icon_url.clone() />
                        <span class="font-bold">{title.clone()}</span>
                    }
                }
            )}
        </a>
    }
}

#[component]
pub fn WebsiteIcon(
    #[prop(into, optional)] class: &'static str,
    #[prop(into)] website: WebsitePreviewResp,
    #[prop(default = 32)] size: u16,
) -> impl IntoView {
    view! {
        <div class=format!("avatar {class}")>
            <div class="rounded" style:width=format!("{size}px") style:height=format!("{size}px")>
                <img alt=website.initials.clone() src=website.icon_image_url(size) />
            </div>
        </div>
    }
}
