use leptos::either::Either;
use leptos::prelude::*;

use crate::models::WebsiteResp;

#[component]
pub fn WebsiteCard(website: WebsiteResp, #[prop(into, optional)] actions: ViewFn) -> impl IntoView {
    let href = move || {
        if website.is_published {
            Some(website.url.clone())
        } else {
            None
        }
    };

    view! {
        <div class="card card-compact bg-base-100 shadow-xl mb-4">
            <div class="card-body">
                <div class="flex gap-3">
                    <a href=href
                        .clone()>
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
                        }
                    </a>

                    <h3 class="card-title">
                        <a href=href>{website.name.clone()}</a>
                    </h3>
                </div>

                <div class="card-text-preview">
                    <div class="prose max-w-none" inner_html=website.description_preview_html />
                    <div class="card-text-preview-overlay" />
                </div>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
