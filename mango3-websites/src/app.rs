use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, BottomBar, FaviconLink};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::{CurrentWebsiteOpt, WebsiteTopBar};
use crate::constants::KEY_PARAM_SLUG;
use crate::context::provide_current_website_resource;
use crate::pages::{IndexPage, ShowPagePage, ShowPostPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_current_website_resource();

    view! {
        <Stylesheet id="leptos" href="/pkg/application.css" />

        <AppProvider>
            <CurrentWebsiteOpt children=move |website| {
                if let Some(icon_image_blob) = website.and_then(|w| w.icon_image_blob) {
                    view! { <FaviconLink href=icon_image_blob.variant_url(32, 32, true) /> }
                } else {
                    view! { <FaviconLink /> }
                }
            } />

            <Router>
                <WebsiteTopBar />

                <main class="grow m-6">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("") view=IndexPage />
                        <Route path=(StaticSegment("posts"), ParamSegment(KEY_PARAM_SLUG)) view=ShowPostPage />
                        <Route path=ParamSegment(KEY_PARAM_SLUG) view=ShowPagePage />
                    </Routes>
                </main>

                <BottomBar />
            </Router>
        </AppProvider>
    }
}
