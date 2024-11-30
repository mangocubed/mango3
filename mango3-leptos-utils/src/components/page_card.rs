use leptos::prelude::*;

use crate::models::PageResp;

#[component]
pub fn PageCard(
    page: PageResp,
    #[prop(into, default = false)] show_content: bool,
    #[prop(into, optional)] actions: ViewFn,
) -> impl IntoView {
    let inner_html = move || {
        if show_content {
            page.content_html.clone()
        } else {
            page.content_preview_html.clone()
        }
    };

    view! {
        <div class="card card-compact bg-base-100 shadow-xl mb-4">
            {
                let page_title = page.title.clone();
                move || {
                    page.cover_image_blob
                        .clone()
                        .map(|cover_image_blob| {
                            view! {
                                <figure>
                                    <img src=cover_image_blob.variant_url(1200, 200, true) alt=page_title.clone() />
                                </figure>
                            }
                        })
                }
            } <div class="card-body">
                <h3 class="card-title">
                    <a href=move || {
                        if !show_content && page.is_published { Some(page.url.clone()) } else { None }
                    }>{page.title}</a>
                </h3>

                <div class="card-text-preview">
                    <div class="prose max-w-none" inner_html=inner_html />
                    <div class="card-text-preview-overlay" />
                </div>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
