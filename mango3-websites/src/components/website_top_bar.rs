use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;

use leptos_meta::Title;
use mango3_leptos_utils::components::{AppTitle, Brand, GoToMango3, TopBar};
use mango3_leptos_utils::context::{use_basic_config, use_page_title};
use mango3_leptos_utils::i18n::use_i18n;

use crate::server_functions::get_all_navigation_items;

use super::CurrentWebsite;

#[component]
pub fn WebsiteTopBar() -> impl IntoView {
    let navigation_items_resource = Resource::new_blocking(|| (), |_| get_all_navigation_items());

    view! {
        <TopBar right_items=move || view! { <GoToMango3 /> }>
            <CurrentWebsite children=move |website| {
                let i18n = use_i18n();
                let basic_config = use_basic_config();
                let page_title = use_page_title();
                match website {
                    Some(website) => {
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

                                <a class="btn btn-ghost text-xl" href="/">
                                    <img
                                        alt=website.name.clone()
                                        class="rounded"
                                        src=website.icon_image_blob.map(|blob| blob.variant_url(42, 42, true))
                                    />
                                    {website.name}
                                </a>

                                <Suspense>
                                    {move || Suspend::new(async move {
                                        navigation_items_resource
                                            .get()
                                            .and_then(|result| result.ok())
                                            .map(|items| {
                                                view! {
                                                    <ul class="menu menu-horizontal">
                                                        <For
                                                            each=move || items.clone()
                                                            key=|item| item.id.clone()
                                                            let:item
                                                        >
                                                            <li>
                                                                <a href=item.url>{item.title}</a>
                                                            </li>
                                                        </For>

                                                    </ul>
                                                }
                                            })
                                    })}
                                </Suspense>
                            },
                        )
                    }
                    None => {
                        Either::Right(move || {
                            let home_url = basic_config.home_url.clone();
                            view! {
                                <AppTitle />

                                <Brand href=home_url.clone() />
                            }
                        })
                    }
                }
            } />
        </TopBar>
    }
}
