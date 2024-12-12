use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::{ChevronDownMini, PlusOutlined};
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_selected_website;

use super::MyWebsitesInfiniteScroll;

#[component]
pub fn WebsiteIcon(website: WebsiteResp) -> impl IntoView {
    move || {
        if let Some(icon_image_blob) = &website.icon_image_blob {
            Either::Left(view! {
                <div class="avatar">
                    <div class="bg-neutral text-neutral-content w-8 rounded-full">
                        <img alt=website.initials.clone() src=icon_image_blob.variant_url(32, 32, true) />
                    </div>
                </div>
            })
        } else {
            Either::Right(view! {
                <div class="avatar placeholder">
                    <div class="bg-neutral text-neutral-content w-8 rounded-full">
                        <span class="text-xs">{website.initials.clone()}</span>
                    </div>
                </div>
            })
        }
    }
}

#[component]
pub fn SelectedWebsiteDropdown() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let selected_website = use_selected_website();

    view! {
        <div class="dropdown">
            <button type="button" class="btn btn-outline btn-block justify-start md:justify-center">
                {move || match selected_website.get() {
                    Some(website) => {
                        Either::Left(
                            view! {
                                <WebsiteIcon website=website.clone() />

                                <div>{website.name}</div>
                            },
                        )
                    }
                    None => Either::Right(t!(i18n, studio.select)),
                }}

                <ChevronDownMini class="hidden md:block" />
            </button>

            <ul class="md:dropdown-content md:menu bg-base-100 rounded-box z-[1] md:w-48 p-2 md:shadow">
                <MyWebsitesInfiniteScroll let:website>
                    <li>
                        <a href=format!("/websites/{}", website.id)>
                            <WebsiteIcon website=website.clone() />
                            <div>{website.name}</div>
                        </a>
                    </li>
                </MyWebsitesInfiniteScroll>

                <li class="disabled">
                    <div class="divider m-0"></div>
                </li>

                <li>
                    <a href=basic_config.new_website_url>
                        <PlusOutlined />

                        {t!(i18n, shared.new_website)}
                    </a>
                </li>
            </ul>
        </div>
    }
}
