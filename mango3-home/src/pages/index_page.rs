use leptos::prelude::*;
use leptos_fluent::tr;

use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::pages::Page;

#[component]
pub fn IndexPage() -> impl IntoView {
    let basic_config = use_basic_config();
    let title = move || tr!("a-cloud-platform-to-create-websites-in-the-easiest-way-possible");

    view! {
        <Page title=title>
            <div class="hero">
                <div class="hero-content text-center flex-col">
                    <h2 class="text-2xl font-bold">
                        {move || tr!("welcome-to-title", { "title" => basic_config.title.clone() })}
                    </h2>
                    <p class="py-3">
                        {title}
                    </p>
                </div>
            </div>
        </Page>
    }
}
