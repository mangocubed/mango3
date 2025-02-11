use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{
    AppProvider, AppTitle, BottomBar, Brand, FaviconLink, GoToMango3, LoadingOverlay, TopBar,
};
use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::IndexPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            {move || {
                let i18n = use_i18n();
                let suffix = move || t_string!(i18n, admin.admin);

                view! {
                    <FaviconLink />

                    <AppTitle suffix=suffix />

                    <Router>
                        <TopBar
                            brand=move || view! { <Brand href="/" suffix=suffix /> }
                            right_items=move || view! { <GoToMango3 /> }
                        />

                        <main class="flex flex-col grow md:m-6 m-4">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>

                    <LoadingOverlay />
                }
            }}
        </AppProvider>
    }
}
