use leptos::prelude::*;

use crate::components::WebsiteIcon;
use crate::models::WebsiteResp;

#[component]
pub fn WebsiteCard(website: WebsiteResp, #[prop(into, optional)] actions: ViewFn) -> impl IntoView {
    let website_url = website.url.clone();

    let href = if website.is_published {
        Some(website_url.clone())
    } else {
        None
    };

    view! {
        <div class="card card-compact bg-base-100 shadow-xl mb-4">
            <div class="card-body">
                <div class="flex gap-3 items-center">
                    <a href=href.clone()>
                        <WebsiteIcon website=website.clone() />
                    </a>

                    <div class="card-title">
                        <a href=href.clone()>
                            <div class="font-bold text-lg">{website.name.clone()}</div>
                            <div class="text-base opacity-70">{website.host.clone()}</div>
                        </a>
                    </div>
                </div>

                <a href=href class="card-text-preview">
                    <div class="prose max-w-none break-words" inner_html=website.description_preview_html />
                    <div class="card-text-preview-overlay to-base-100" />
                </a>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
