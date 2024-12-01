use leptos::ev::Event;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::{Link, Title};

use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};

mod action_form_alert;
mod alert_dialog;
mod app_provider;
mod authentication;
mod bottom_bar;
mod confirmation_dialog;
mod country_field;
mod current_user_resource;
mod image_upload_field;
mod infinite_scroll;
mod loading_spinner;
mod page_card;
mod password_field;
mod post_card;
mod submit_button;
mod switch_field;
mod text_field;
mod textarea_field;
mod top_bar;
mod website_card;

pub use action_form_alert::{ActionFormAlert, ActionFormError};
pub use alert_dialog::AlertDialog;
pub use app_provider::AppProvider;
pub use authentication::{RequireAuthentication, RequireNoAuthentication};
pub use bottom_bar::BottomBar;
pub use confirmation_dialog::ConfirmationDialog;
pub use country_field::CountryField;
pub use current_user_resource::CurrentUserResource;
pub use image_upload_field::ImageUploadField;
pub use infinite_scroll::InfiniteScroll;
pub use loading_spinner::LoadingSpinner;
pub use page_card::PageCard;
pub use password_field::PasswordField;
pub use post_card::PostCard;
pub use submit_button::SubmitButton;
pub use switch_field::SwitchField;
pub use text_field::TextField;
pub use textarea_field::TextareaField;
pub use top_bar::TopBar;
pub use website_card::WebsiteCard;

pub struct EventFn(Box<dyn Fn(Event) + 'static>);

impl<T> From<T> for EventFn
where
    T: Fn(Event) + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

pub struct BoxedFn(Box<dyn Fn() + 'static>);

impl<T> From<T> for BoxedFn
where
    T: Fn() + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[component]
pub fn AppTitle(#[prop(optional, into)] suffix: Option<TextProp>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <Title formatter=move |page_title: String| {
            let mut text = (if page_title.is_empty() { String::new() } else { format!("{page_title} | ") })
                + &basic_config.title;
            if let Some(suffix) = &suffix { text + &format!(" {}", suffix.get()) } else { text }
        } />
    }
}

#[component]
pub fn Brand(#[prop(into)] href: String, #[prop(optional, into)] suffix: Option<TextProp>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <a class="btn btn-ghost text-xl" href=href>
            <img class="h-[36px]" src=basic_config.asset_url("logo.svg") alt=basic_config.title.clone() />
            {move || suffix.as_ref().map(|suffix| suffix.get())}
        </a>
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
        <a class="btn btn-ghost" href=basic_config.home_url.clone()>
            {t!(i18n, shared.go_to_title, title = basic_config.title.clone())}
        </a>
    }
}
