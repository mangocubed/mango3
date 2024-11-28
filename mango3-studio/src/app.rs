use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Meta, Stylesheet};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, FaviconLink, GoToMango3, TopBar};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;

use crate::constants::{KEY_PARAM_PAGE_ID, KEY_PARAM_POST_ID, KEY_PARAM_WEBSITE_ID};
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
                let suffix = move || t_string!(i18n, shared.studio);

                view! {
                    <FaviconLink />

                    <AppTitle suffix=suffix />

                    <Router>
                        <TopBar right_items=move || view! { <GoToMango3 /> }>
                            <Brand href="/" suffix=suffix />
                        </TopBar>

                        <main class="flex flex-col grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=StaticSegment("new-website") view=NewWebsitePage />
                                <ParentRoute
                                    path=(StaticSegment("websites"), ParamSegment(KEY_PARAM_WEBSITE_ID))
                                    view=websites::ShowParentPage
                                >
                                    <Route path=StaticSegment("") view=websites::ShowPage />
                                    <Route path=StaticSegment("posts") view=websites::PostsPage />
                                    <Route
                                        path=(StaticSegment("posts"), StaticSegment("new"))
                                        view=websites::NewPostPage
                                    />
                                    <Route
                                        path=(
                                            StaticSegment("posts"),
                                            ParamSegment(KEY_PARAM_POST_ID),
                                            StaticSegment("edit"),
                                        )
                                        view=websites::EditPostPage
                                    />
                                    <Route path=StaticSegment("pages") view=websites::PagesPage />
                                    <Route
                                        path=(StaticSegment("pages"), StaticSegment("new"))
                                        view=websites::NewPagePage
                                    />
                                    <Route
                                        path=(
                                            StaticSegment("pages"),
                                            ParamSegment(KEY_PARAM_PAGE_ID),
                                            StaticSegment("edit"),
                                        )
                                        view=websites::EditPagePage
                                    />
                                    <Route path=StaticSegment("navigation") view=websites::NavigationPage />
                                    <Route path=StaticSegment("edit") view=websites::EditPage />
                                </ParentRoute>
                            </Routes>
                        </main>

                        <BottomBar />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
