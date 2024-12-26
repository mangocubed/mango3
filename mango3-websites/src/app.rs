use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, FaviconLink};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::{CurrentWebsite, CurrentWebsiteOpt, WebsiteTopBar};
use crate::constants::KEY_PARAM_SLUG;
use crate::context::provide_current_website_resource;
use crate::pages::{IndexPage, SearchPage, ShowPagePage, ShowPostPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_current_website_resource();

    view! {
        <AppProvider>
            {move || {
                let i18n = use_i18n();
                let basic_config = use_basic_config();
                let title = basic_config.title.clone();
                view! {
                    <CurrentWebsiteOpt children={
                        let title = title.clone();
                        move |website| {
                            match website {
                                Some(website) => {
                                    let title = title.clone();
                                    Either::Left(
                                        view! {
                                            <Title formatter=move |page_title: String| {
                                                (if page_title.is_empty() {
                                                    String::new()
                                                } else {
                                                    format!("{page_title} | ")
                                                })
                                                    + &format!(
                                                        "{} ({})",
                                                        website.name.clone(),
                                                        t_string!(
                                                            i18n, websites.powered_by_title, title = title.clone()
                                                        ),
                                                    )
                                            } />

                                            {move || {
                                                if let Some(icon_image_blob) = &website.icon_image_blob {
                                                    view! {
                                                        <FaviconLink href=icon_image_blob.variant_url(32, 32, true) />
                                                    }
                                                } else {
                                                    view! { <FaviconLink /> }
                                                }
                                            }}
                                        },
                                    )
                                }
                                None => Either::Right(view! { <AppTitle /> }),
                            }
                        }
                    } />

                    <Router>
                        <WebsiteTopBar />

                        <main class="grow m-6">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=(StaticSegment("posts"), ParamSegment(KEY_PARAM_SLUG)) view=ShowPostPage />
                                <Route path=StaticSegment("search") view=SearchPage />
                                <Route path=ParamSegment(KEY_PARAM_SLUG) view=ShowPagePage />
                            </Routes>
                        </main>

                        <CurrentWebsiteOpt children=move |website| {
                            match website {
                                Some(website) => {
                                    let title = title.clone();
                                    Either::Left(
                                        view! {
                                            <BottomBar
                                                light_theme=website.light_theme.clone()
                                                dark_theme=website.dark_theme.clone()
                                                aside_items=move || {
                                                    view! {
                                                        <CurrentWebsite let:_>
                                                            {t!(
                                                                i18n, websites.this_website_is_part_of_title_ecosystem, title = title.clone()
                                                            )}
                                                        </CurrentWebsite>
                                                    }
                                                }
                                            />
                                        },
                                    )
                                }
                                None => Either::Right(view! { <BottomBar /> }),
                            }
                        } />
                    </Router>
                }
            }}
        </AppProvider>
    }
}
