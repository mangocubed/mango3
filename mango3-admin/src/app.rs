use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{
    AppProvider, AppTitle, BottomBar, Brand, FaviconLink, GoToMango3, LoadingOverlay, TopBar,
};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::pages::{IndexPage, IndexParentPage, UsersPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            {move || {
                let i18n = use_i18n();
                let text_suffix = async_t_string!(i18n, admin.admin).to_signal();

                view! {
                    <FaviconLink />

                    <AppTitle suffix=text_suffix />

                    <Router>
                        <TopBar
                            brand=move || view! { <Brand href="/" suffix=text_suffix /> }
                            right_items=move |_| view! { <GoToMango3 /> }
                        />

                        <main class="flex flex-col grow md:m-6 m-4">
                            <Routes fallback=NotFoundPage>
                                <ParentRoute path=StaticSegment("") view=IndexParentPage>
                                    <Route path=StaticSegment("") view=IndexPage />
                                    <Route path=StaticSegment("users") view=UsersPage />
                                </ParentRoute>
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
