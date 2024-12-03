use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, FaviconLink, TopBar};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::pages::{IndexPage, PostsPage, WebsitesPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <AppProvider>
            {move || {
                let basic_config = use_basic_config();
                let i18n = use_i18n();

                view! {
                    <FaviconLink />

                    <AppTitle />

                    <Meta
                        name="description"
                        content=move || {
                            t_string!(i18n, home.a_cloud_platform_to_create_websites_in_the_easiest_way_possible)
                        }
                    />

                    <Meta name="copyright" content=basic_config.copyright.clone() />

                    <Router>
                        <TopBar>
                            <Brand href="/" />

                            <a class="btn btn-ghost" href="/">
                                {t!(i18n, shared.home)}
                            </a>

                            <a class="btn btn-ghost" href="/posts">
                                {t!(i18n, shared.posts)}
                            </a>

                            <a class="btn btn-ghost" href="/websites">
                                {t!(i18n, home.websites)}
                            </a>
                        </TopBar>

                        <main class="grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=StaticSegment("posts") view=PostsPage />
                                <Route path=StaticSegment("websites") view=WebsitesPage />
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
