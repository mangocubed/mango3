use leptos::prelude::*;

use crate::context::use_basic_config;

#[component]
pub fn Brand(#[prop(into)] href: String, #[prop(optional, into)] suffix: Signal<Option<String>>) -> impl IntoView {
    let basic_config = use_basic_config();

    view! {
        <a class="btn btn-ghost btn-lg text-xl px-2" href=href title=basic_config.title.clone()>
            <picture>
                <source media="(min-width: 768px)" srcset=basic_config.asset_url("logo.svg").to_string() />
                <img
                    alt=basic_config.title.clone()
                    class="h-[36px]"
                    src=basic_config.asset_url("icon.svg").to_string()
                />
            </picture>

            {move || suffix.get()}
        </a>
    }
}
