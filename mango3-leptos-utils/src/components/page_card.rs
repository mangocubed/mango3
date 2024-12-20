use leptos::prelude::*;

use crate::i18n::{t, use_i18n};
use crate::models::PagePreviewResp;

use super::TimeAgo;

#[component]
pub fn PageCard(page: PagePreviewResp, #[prop(into, optional)] actions: ViewFn) -> impl IntoView {
    let i18n = use_i18n();

    let href = move || {
        if page.is_published {
            Some(page.url.clone())
        } else {
            None
        }
    };

    view! {
        <div class="card card-compact bg-base-200 shadow-xl mb-4">
            {
                let page_title = page.title.clone();
                let href = href.clone();
                move || {
                    page.cover_image_blob
                        .clone()
                        .map(|cover_image_blob| {
                            view! {
                                <figure>
                                    <a href=href.clone() title=page_title.clone()>
                                        <img src=cover_image_blob.variant_url(1200, 200, true) alt=page_title.clone() />
                                    </a>
                                </figure>
                            }
                        })
                }
            } <div class="card-body">
                <h3 class="card-title">
                    <a href=href.clone()>{page.title}</a>
                </h3>

                <div class="self-end text-right my-1">
                    <TimeAgo value=page.created_at />

                    {move || {
                        page.updated_at
                            .map(|update_at| {
                                view! {
                                    " ("
                                    {t!(i18n, shared.edited)}
                                    " "
                                    <TimeAgo value=update_at />
                                    ")"
                                }
                            })
                    }}
                </div>

                <a href=href class="card-text-preview">
                    <div class="prose max-w-none" inner_html=page.content_preview_html />
                    <div class="card-text-preview-overlay to-base-200" />
                </a>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
