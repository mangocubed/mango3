use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

use crate::context::{provide_basic_config_resource, provide_page_title};

static_loader! {
    static TRANSLATIONS = {
        locales: "../locales",
        fallback_language: "en",
    };
}

#[component]
pub fn AppProvider(children: Children) -> impl IntoView {
    provide_basic_config_resource();
    provide_page_title();

    let is_done = RwSignal::new(false);

    Effect::new(move || is_done.set(true));

    view! {
        <div class="loading-overlay" class:is-done=is_done>
            <figure>
                <div class="pulse"></div>
                <img src="/icon.svg" />
            </figure>
        </div>

        <LeptosFluent>
            <div class="flex flex-col min-h-screen">{children()}</div>
        </LeptosFluent>
    }
}

#[component]
fn LeptosFluent(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../locales",
        translations: [TRANSLATIONS],

        // #[cfg(debug_assertions)]
        // check_translations: "../**/*.rs",

        initial_language_from_cookie: true,
        cookie_name: "_mango3_language",
        set_language_to_cookie: true,

        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
    }
}
