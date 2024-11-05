use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

use crate::{
    constants::COOKIE_NAME_LANGUAGE,
    context::{provide_basic_config, provide_page_title, use_basic_config},
};

static_loader! {
    static LOCALES = {
        locales: "../locales",
        fallback_language: "en",
    };
}

#[component]
pub fn AppProvider(children: Children) -> impl IntoView {
    provide_basic_config();
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
    #[allow(unused_variables)]
    let basic_config = use_basic_config();

    leptos_fluent! {
        children: children(),
        locales: "../locales",
        translations: [LOCALES],

        // #[cfg(debug_assertions)]
        // check_translations: "../**/*.rs",

        initial_language_from_cookie: true,
        cookie_name: COOKIE_NAME_LANGUAGE,
        cookie_attrs: &format!("Path=/; SameSite=Strict; Domain={}", basic_config.domain),
        set_language_to_cookie: true,

        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
    }
}
