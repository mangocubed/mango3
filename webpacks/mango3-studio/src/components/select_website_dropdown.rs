use leptos::either::Either;
use leptos::prelude::*;

use mango3_web_utils::components::{CurrentUser, WebsiteIcon};
use mango3_web_utils::context::use_basic_config;
use mango3_web_utils::enums::Orientation;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::{ChevronDownMini, PlusOutlined};

use crate::context::use_selected_website;

use super::MyWebsitesInfiniteScroll;

#[component]
pub fn SelectWebsiteDropdown(orientation: Orientation) -> impl IntoView {
    let i18n = use_i18n();
    let selected_website = use_selected_website();

    let is_horizontal = orientation.is_horizontal();

    view! {
        <div class:dropdown=is_horizontal>
            <button type="button" class="btn btn-outline btn-accent btn-block justify-between">
                {move || match selected_website.get() {
                    Some(website) => {
                        Either::Left(
                            view! {
                                <WebsiteIcon website=website.clone() />

                                <div>{website.name}</div>
                            },
                        )
                    }
                    None => Either::Right(t!(i18n, shared.select)),
                }}

                <ChevronDownMini />
            </button>

            <ul
                class="bg-base-100 rounded-box z-[1] p-2 md:w-60"
                class:menu=is_horizontal
                class:shadow=is_horizontal
                class:dropdown-content=is_horizontal
            >
                <MyWebsitesInfiniteScroll let:website>
                    <li>
                        <a href=format!("/websites/{}", website.id)>
                            <WebsiteIcon website=website.clone() />
                            <div>{website.name}</div>
                        </a>
                    </li>
                </MyWebsitesInfiniteScroll>

                <CurrentUser let:user>
                    <Show when=move || {
                        user.can_insert_website
                    }>
                        {move || {
                            let basic_config = use_basic_config();
                            view! {
                                <li class="disabled">
                                    <div class="divider m-0"></div>
                                </li>

                                <li>
                                    <a href=basic_config.new_website_url.to_string()>
                                        <PlusOutlined />

                                        {t!(i18n, shared.new_website)}
                                    </a>
                                </li>
                            }
                        }}
                    </Show>
                </CurrentUser>
            </ul>
        </div>
    }
}
