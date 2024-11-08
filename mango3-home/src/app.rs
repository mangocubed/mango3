use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, TopBar};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::IndexPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <AppProvider>
            {move || {
                let basic_config = use_basic_config();
                let i18n = use_i18n();

                view! {
                    <AppTitle />

                    <Meta
                        name="description"
                        content=move || {
                            t_string!(
                                i18n, home.a_cloud_platform_to_create_websites_in_the_easiest_way_possible
                            )
                        }
                    />

                    <Meta name="copyright" content=basic_config.copyright.clone() />

                    <Router>
                        <TopBar>
                            <Brand href="/" />

                            <a class="btn btn-ghost" href="/">
                                {t!(i18n, shared.home)}
                            </a>
                        </TopBar>

                        <main class="grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
