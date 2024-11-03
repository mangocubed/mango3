use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BasicConfigResource, BottomBar, Brand, TopBar};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::IndexPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <AppProvider>
            <Meta
                name="description"
                content=move || {
                    tr!("a-cloud-platform-to-create-websites-in-the-easiest-way-possible")
                }
            />

            <AppTitle />

            <BasicConfigResource let:basic_config>
                <Meta name="copyright" content=basic_config.copyright />
            </BasicConfigResource>

            <Router>
                <TopBar>
                    <Brand href="/" />

                    <a class="btn btn-ghost" href="/">
                        {move || tr!("home")}
                    </a>
                </TopBar>

                <main class="grow m-6">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("") view=IndexPage />
                    </Routes>
                </main>

                <BottomBar />
            </Router>
        </AppProvider>
    }
}
