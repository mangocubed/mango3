use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, GoToMango3, TopBar};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::RegisterPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            <AppTitle suffix=move || tr!("accounts") />

            <Router>
                <TopBar right_items=move || view! { <GoToMango3 /> }>
                    <Brand href="/register" suffix=move || tr!("accounts") />
                </TopBar>

                <main class="grow m-6">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("register") view=RegisterPage />
                    </Routes>
                </main>

                <BottomBar />
            </Router>
        </AppProvider>
    }
}
