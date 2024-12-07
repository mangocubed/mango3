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
            <button tabindex="0" type="button" class="btn btn-outline">
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

                <ChevronDownMini />
            </button>

            <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow">
                <MyWebsitesInfiniteScroll let:website>
                    <li>
                        <a href=format!("/websites/{}", website.id)>
                            <WebsiteIcon website=website.clone() />
                            <div>{website.name}</div>
                        </a>
                    </li>
                </MyWebsitesInfiniteScroll>

                <li class="disabled">
                    <hr class="bg-gray-500 border-none h-[2px] m-2 p-0" />
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
