use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::Page;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let basic_config = use_basic_config();
    let config_title = basic_config.title.clone();

    let page_title = move || {
        t_string!(
            i18n,
            home.a_cloud_platform_to_create_websites_in_the_easiest_way_possible
        )
    };

    view! {
        <Page title=page_title>
            <div class="hero">
                <div class="hero-content text-center flex-col">
                    <h2 class="text-2xl font-bold">
                        {t!(i18n, home.welcome_to_title, title = config_title)}
                    </h2>
                    <p class="py-3">{page_title}</p>
                </div>
            </div>
        </Page>
    }
}
