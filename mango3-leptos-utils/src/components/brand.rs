use leptos::prelude::*;
use leptos::text_prop::TextProp;

use crate::context::use_basic_config;

#[component]
pub fn Brand(#[prop(into)] href: String, #[prop(optional, into)] suffix: Option<TextProp>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <a class="btn btn-ghost text-xl px-2" href=href title=basic_config.title.clone()>
            <picture>
                <source media="(min-width: 768px)" srcset=basic_config.asset_url("logo.svg") />
                <img alt=basic_config.title.clone() class="h-[36px]" src=basic_config.asset_url("icon.svg") />
            </picture>

            {move || suffix.as_ref().map(|suffix| suffix.get())}
        </a>
    }
}
