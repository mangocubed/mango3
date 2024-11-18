use leptos::either::Either;
use leptos::prelude::*;

use crate::i18n::{t, use_i18n};
use crate::models::WebsiteResp;

#[component]
pub fn WebsiteCard(website: WebsiteResp, #[prop(into, optional)] actions: ViewFn) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="card card-compact bg-base-100 shadow-xl mb-4">
            <div class="card-body">
                <div class="flex gap-3">
                    {
                        let website_name = website.name.clone();
                        move || {
                            if let Some(icon_image_blob) = &website.icon_image_blob {
                                Either::Left(
                                    view! {
                                        <div class="avatar">
                                            <div class="w-[32px] rounded">
                                                <img
                                                    alt=website_name.clone()
                                                    class="rounded"
                                                    width=32
                                                    height=32
                                                    src=icon_image_blob.variant_url(32, 32, true)
                                                />
                                            </div>
                                        </div>
                                    },
                                )
                            } else {
                                Either::Right(
                                    view! {
                                        <div class="avatar placeholder">
                                            <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                                <span class="text-xs">{website.initials.clone()}</span>
                                            </div>
                                        </div>
                                    },
                                )
                            }
                        }
                    } <h3 class="card-title">{website.name.clone()}</h3>
                </div>

                <p>{website.description}</p>

                <div class="card-actions justify-end">
                    <Show when=move || website.is_published>
                        <a class="btn btn-ghost font-bold" href=website.url.clone()>
                            {t!(i18n, shared.go_to_website)}
                        </a>
                    </Show>

                    {actions.run()}
                </div>
            </div>
        </div>
    }
}
