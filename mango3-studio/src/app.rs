use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{
    AppProvider, AppTitle, BottomBar, Brand, FaviconLink, GoToMango3, LoadingOverlay, TopBar, UnconfirmedEmailAlert,
};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::SelectWebsiteDropdown;
use crate::constants::{KEY_PARAM_POST_ID, KEY_PARAM_WEBSITE_ID};
use crate::context::provide_selected_website;
use crate::pages::{websites, IndexPage, NewWebsitePage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Meta name="robots" content="noindex, nofollow" />

        <AppProvider>
            {move || {
                provide_selected_website();
                let i18n = use_i18n();
                let text_suffix = async_t_string!(i18n, shared.studio).to_signal();

                view! {
                    <FaviconLink />

                    <AppTitle suffix=text_suffix />

                    <Router>
                        <TopBar
                            brand=move || view! { <Brand href="/" suffix=text_suffix /> }
                            left_items=move |orientation| view! { <SelectWebsiteDropdown orientation=orientation /> }
                            right_items=move |_| view! { <GoToMango3 /> }
                        />

                        <UnconfirmedEmailAlert />

                        <main class="flex flex-col grow md:m-6 m-4">
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
                                    <Route path=StaticSegment("files") view=websites::FilesPage />
                                    <Route path=StaticSegment("navigation") view=websites::NavigationPage />
                                    <Route path=StaticSegment("edit") view=websites::EditPage />
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
