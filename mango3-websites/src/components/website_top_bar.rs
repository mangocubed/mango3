use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::{Brand, GoToMango3, SearchBar, TopBar, WebsiteIcon};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::enums::Orientation;

use crate::server_functions::get_all_navigation_items;

use super::CurrentWebsiteOpt;

#[component]
pub fn WebsiteTopBar() -> impl IntoView {
    let navigation_items_resource = Resource::new_blocking(|| (), |_| get_all_navigation_items());

    view! {
        <TopBar
            brand=move || {
                view! {
                    <CurrentWebsiteOpt children=move |website| {
                        let basic_config = use_basic_config();
                        match website {
                            Some(website) => {
                                Either::Left(
                                    view! {
                                        <a
                                            class="btn btn-ghost btn-lg text-xl pl-1 pr-2"
                                            href="/"
                                            title=website.name.clone()
                                        >
                                            <WebsiteIcon website=website.clone() size=42 />

                                            {website.name.clone()}
                                        </a>
                                    },
                                )
                            }
                            None => {
                                let home_url = basic_config.home_url.clone();
                                Either::Right(view! { <Brand href=home_url.clone() /> })
                            }
                        }
                    } />
                }
            }
            class="bg-base-200"
            left_items=move |orientation: Orientation| {
                let is_horizontal = orientation.is_horizontal();

                view! {
                    <Transition>
                        {move || Suspend::new(async move {
                            navigation_items_resource
                                .get()
                                .and_then(|result| result.ok())
                                .map(|items| {
                                    view! {
                                        <ul class="menu gap-1" class:menu-horizontal=is_horizontal>
                                            <For each=move || items.clone() key=|item| item.id.clone() let:item>
                                                <li>
                                                    <a href=item.url>{item.title}</a>
                                                </li>
                                            </For>

                                            <li>
                                                <SearchBar />
                                            </li>
                                        </ul>
                                    }
                                })
                        })}
                    </Transition>
                }
            }
            right_items=move |_| view! { <GoToMango3 /> }
        />
    }
}
