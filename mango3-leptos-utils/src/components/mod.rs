use leptos::prelude::*;
use leptos_meta::Title;

use crate::context::use_page_title;

mod app_provider;
mod basic_config_resource;
mod bottom_bar;
mod top_bar;

pub use app_provider::AppProvider;
pub use basic_config_resource::BasicConfigResource;
pub use bottom_bar::BottomBar;
pub use top_bar::TopBar;

#[component]
pub fn AppTitle(#[prop(optional, into)] suffix: Option<&'static str>) -> impl IntoView {
    let page_title = use_page_title();

    view! {
        <BasicConfigResource children=move |basic_config| {
            let title_text = move || {
                let mut text = "".to_owned();
                if let Some(page_title) = page_title.value.get() {
                    text += &format!("{page_title} | ");
                }
                text += &basic_config.title;
                if let Some(suffix) = suffix {
                    text += &format!(" {suffix}");
                }
                text
            };

            view! { <Title text=title_text /> }
        } />
    }
}

#[component]
pub fn Brand(href: &'static str, #[prop(optional, into)] suffix: &'static str) -> impl IntoView {
    view! {
        <BasicConfigResource children=move |basic_config| {
            view! {
                <a class="btn btn-ghost text-xl" href=href>
                    <img
                        class="h-[36px]"
                        src="/logo.svg"
                        alt=format!("{} {}", basic_config.title, suffix)
                    />
                    {suffix}
                </a>
            }
        } />
    }
}
