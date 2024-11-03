use leptos::prelude::*;
use leptos_fluent::tr;

use mango3_leptos_utils::components::BasicConfigResource;
use mango3_leptos_utils::pages::Page;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <Page title=move || tr!("a-cloud-platform-to-create-websites-in-the-easiest-way-possible")>
            <div class="hero">
                <div class="hero-content text-center flex-col">
                    <h2 class="text-2xl font-bold">
                        <BasicConfigResource children=move |basic_config| {
                            tr!("welcome-to-title", { "title" => basic_config.title })
                        } />
                    </h2>
                    <p class="py-3">
                        {move || {
                            tr!("a-cloud-platform-to-create-websites-in-the-easiest-way-possible")
                        }}
                    </p>
                </div>
            </div>
        </Page>
    }
}
