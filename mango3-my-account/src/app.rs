use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, GoToMango3, TopBar};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::IndexPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            <AppTitle suffix=move || tr!("my-account") />

            <Router>
                <TopBar right_items=move || view! { <GoToMango3 /> }>
                    <Brand href="/" suffix=move || tr!("my-account") />
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
