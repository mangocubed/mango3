use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{
    AppProvider, AppTitle, BottomBar, Brand, FaviconLink, LoadingOverlay, SearchBar, TopBar,
};
use mango3_leptos_utils::constants::KEY_PARAM_NAME;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::constants::KEY_PARAM_USERNAME;
use crate::pages::{IndexPage, PostsPage, SearchPage, ShowHashtagPage, ShowUserPage, WebsitesPage};

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

                    <Meta name="description" content=basic_config.description.clone() />

                    <Meta name="copyright" content=basic_config.copyright.clone() />

                    <Router>
                        <TopBar
                            brand=move || view! { <Brand href="/" /> }
                            left_items=move |_| {
                                view! {
                                    <ul class="menu md:menu-horizontal gap-1">
                                        <li>
                                            <a href="/posts">{t!(i18n, shared.posts)}</a>
                                        </li>

                                        <li>
                                            <a href="/websites">{t!(i18n, home.websites)}</a>
                                        </li>

                                        <li>
                                            <SearchBar />
                                        </li>
                                    </ul>
                                }
                            }
                        />

                        <main class="grow md:m-6 m-4">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=StaticSegment("posts") view=PostsPage />
                                <Route path=StaticSegment("websites") view=WebsitesPage />
                                <Route path=StaticSegment("search") view=SearchPage />
                                <Route
                                    path=(StaticSegment("hashtags"), ParamSegment(KEY_PARAM_NAME))
                                    view=ShowHashtagPage
                                />
                                <Route
                                    path=(StaticSegment("users"), ParamSegment(KEY_PARAM_USERNAME))
                                    view=ShowUserPage
                                />
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
