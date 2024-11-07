use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_fluent::tr;
use leptos_meta::Title;

use crate::context::{use_basic_config, use_page_title};

mod action_form_alert;
mod alert_dialog;
mod app_provider;
mod authentication;
mod bottom_bar;
mod confirmation_dialog;
mod country_field;
mod current_user_resource;
mod image_upload_field;
mod password_field;
mod submit_button;
mod text_field;
mod top_bar;

pub use action_form_alert::ActionFormAlert;
pub use alert_dialog::AlertDialog;
pub use app_provider::AppProvider;
pub use authentication::{RequireAuthentication, RequireNoAuthentication};
pub use bottom_bar::BottomBar;
pub use confirmation_dialog::ConfirmationDialog;
pub use country_field::CountryField;
pub use current_user_resource::CurrentUserResource;
pub use image_upload_field::ImageUploadField;
pub use password_field::PasswordField;
pub use submit_button::SubmitButton;
pub use text_field::TextField;
pub use top_bar::TopBar;

#[component]
pub fn AppTitle(#[prop(optional, into)] suffix: Option<TextProp>) -> impl IntoView {
    let basic_config = use_basic_config();
    let page_title = use_page_title();

    let title_text = move || {
        let mut text = "".to_owned();
        if let Some(page_title) = page_title.value.get() {
            text += &format!("{page_title} | ");
        }
        text += &basic_config.title;
        if let Some(suffix) = suffix.clone() {
            text += &format!(" {}", suffix.get());
        }
        text
    };

    view! { <Title text=title_text /> }
}

#[component]
pub fn Brand(href: &'static str, #[prop(optional, into)] suffix: Option<TextProp>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <a class="btn btn-ghost text-xl" href=href>
            <img class="h-[36px]" src="/logo.svg" alt=basic_config.title.clone() />
            {move || suffix.as_ref().map(|suffix| suffix.get())}
        </a>
    }
}

#[component]
pub fn GoToMango3() -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <a class="btn btn-ghost" href=basic_config.home_url.clone()>
            {
                let basic_config = basic_config.clone();
                move || tr!("go-to-title", { "title" => basic_config.title.clone() })
            }
        </a>
    }
}
