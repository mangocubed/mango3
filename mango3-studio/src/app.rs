use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, GoToMango3, TopBar};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;

use crate::constants::KEY_PARAM_WEBSITE_ID;
use crate::pages::{websites, IndexPage, NewWebsitePage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            {move || {
                let i18n = use_i18n();
                let suffix = move || t_string!(i18n, studio.studio);
                view! {
                    <AppTitle suffix=suffix />

                    <Router>
                        <TopBar right_items=move || view! { <GoToMango3 /> }>
                            <Brand href="/" suffix=suffix />
                        </TopBar>

                        <main class="grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=StaticSegment("new-website") view=NewWebsitePage />
                                <Route
                                    path=(
                                        StaticSegment("websites"),
                                        ParamSegment(KEY_PARAM_WEBSITE_ID),
                                    )
                                    view=websites::ShowPage
                                />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
