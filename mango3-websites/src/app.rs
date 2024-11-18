use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, BottomBar};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::WebsiteTopBar;
use crate::context::provide_current_website_resource;
use crate::pages::{IndexPage, ShowPostPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_current_website_resource();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <AppProvider>
            <Router>
                <WebsiteTopBar />

                <main class="grow m-6">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("") view=IndexPage />
                        <Route path=(StaticSegment("posts"), ParamSegment("slug")) view=ShowPostPage />
                    </Routes>
                </main>

                <BottomBar />
            </Router>
        </AppProvider>
    }
}
