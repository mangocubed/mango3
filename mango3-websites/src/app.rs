use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, FaviconLink, LoadingOverlay};
use mango3_leptos_utils::constants::KEY_PARAM_NAME;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::{CurrentWebsiteOpt, WebsiteTopBar};
use crate::constants::KEY_PARAM_SLUG;
use crate::context::provide_current_website_resource;
use crate::pages::{IndexPage, SearchPage, ShowHashtagPage, ShowPostPage};

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
                let text_powered_by_mango3 = async_t_string!(
                    i18n, websites.powered_by_title, title = basic_config.title.clone()
                );
                view! {
                    <CurrentWebsiteOpt children=move |website| {
                        match website {
                            Some(website) => {
                                let website_name = website.name.clone();
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
                                                    website_name.clone(),
                                                    text_powered_by_mango3
                                                        .with(|value| {
                                                            value.clone().unwrap_or("Powered by Mango³".to_owned())
                                                        }),
                                                )
                                        } />

                                        <FaviconLink href=website.icon_image_url(32) />
                                    },
                                )
                            }
                            None => Either::Right(view! { <AppTitle /> }),
                        }
                    } />

                    <Router>
                        <WebsiteTopBar />

                        <main class="grow md:m-6 m-4">
                            <Routes fallback=NotFoundPage>
                                <Route path=StaticSegment("") view=IndexPage />
                                <Route path=(StaticSegment("posts"), ParamSegment(KEY_PARAM_SLUG)) view=ShowPostPage />
                                <Route path=StaticSegment("search") view=SearchPage />
                                <Route
                                    path=(StaticSegment("hashtags"), ParamSegment(KEY_PARAM_NAME))
                                    view=ShowHashtagPage
                                />
                                <Route path=ParamSegment(KEY_PARAM_SLUG) view=ShowPostPage />
                            </Routes>
                        </main>

                        <CurrentWebsiteOpt children=move |website| {
                            match website {
                                Some(website) => {
                                    Either::Left(
                                        view! {
                                            <BottomBar
                                                light_theme=website.light_theme.clone()
                                                dark_theme=website.dark_theme.clone()
                                                aside_items=move || {
                                                    let basic_config = use_basic_config();
                                                    t!(
                                                        i18n,
                                                        websites.this_website_is_part_of_title_ecosystem,
                                                        title = basic_config.title.clone()
                                                    )
                                                }
                                            />
                                        },
                                    )
                                }
                                None => {
                                    Either::Right(

                                        view! { <BottomBar /> },
                                    )
                                }
                            }
                        } />
                    </Router>

                    <CurrentWebsiteOpt children=move |website| {
                        match website {
                            Some(website) => {
                                Either::Left(
                                    view! {
                                        <LoadingOverlay
                                            icon_class="rounded-xl"
                                            icon_url=website.icon_image_url(128)
                                            pulse_class="rounded"
                                        />
                                    },
                                )
                            }
                            None => Either::Right(view! { <LoadingOverlay /> }),
                        }
                    } />
                }
            }}
        </AppProvider>
    }
}
