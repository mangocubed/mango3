use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{ParamSegment, StaticSegment};

use mango3_leptos_utils::components::{AppProvider, AppTitle, BottomBar, Brand, GoToMango3, TopBar};
use mango3_leptos_utils::context::{use_basic_config, use_page_title};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::CurrentWebsiteResource;
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
                <CurrentWebsiteResource children=move |website| {
                    let basic_config = use_basic_config();
                    match website {
                        Some(website) => {
                            let i18n = use_i18n();
                            let page_title = use_page_title();
                            let website_name = website.name.clone();
                            let title_text = move || {
                                let mut text = "".to_owned();
                                if let Some(page_title) = page_title.value.get() {
                                    text += &format!("{page_title} | ");
                                }
                                text
                                    + &format!(
                                        "{} ({})",
                                        website_name.clone(),
                                        t_string!(i18n, websites.powered_by_title, title = basic_config.title.clone()),
                                    )
                            };
                            Either::Left(
                                view! {
                                    <Title text=title_text />

                                    {
                                        let website_icon_image_blob = website.icon_image_blob.clone();
                                        move || {
                                            website_icon_image_blob
                                                .clone()
                                                .map(|blob| {
                                                    view! { <Link rel="icon" href=blob.variant_url(32, 32, true) /> }
                                                })
                                        }
                                    }

                                    <TopBar right_items=move || {
                                        view! { <GoToMango3 /> }
                                    }>
                                        <a class="btn btn-ghost text-xl" href="/">
                                            <img
                                                alt=website.name.clone()
                                                class="rounded"
                                                src=website.icon_image_blob.map(|blob| blob.variant_url(42, 42, true))
                                            />
                                            {website.name}
                                        </a>
                                    </TopBar>

                                    <main class="grow m-6">
                                        <Routes fallback=NotFoundPage>
                                            <Route path=StaticSegment("") view=IndexPage />
                                            <Route path=ParamSegment("slug") view=ShowPostPage />
                                        </Routes>
                                    </main>
                                },
                            )
                        }
                        None => {
                            Either::Right(move || {
                                let home_url = basic_config.home_url.clone();
                                view! {
                                    <AppTitle />

                                    <TopBar right_items=move || view! { <GoToMango3 /> }>
                                        <Brand href=home_url.clone() />
                                    </TopBar>

                                    <main class="grow m-6">
                                        <NotFoundPage />
                                    </main>
                                }
                            })
                        }
                    }
                } />

                <BottomBar />
            </Router>
        </AppProvider>
    }
}
