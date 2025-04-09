use leptos::either::Either;
use leptos::prelude::*;

use crate::components::Hashtags;
use crate::components::WebsiteIcon;
use crate::i18n::{t, use_i18n};
use crate::presenters::WebsiteMinPresenter;

#[component]
pub fn WebsiteCard(
    #[prop(into)] website: WebsiteMinPresenter,
    #[prop(into, optional)] actions: ViewFn,
    #[prop(default = "/".to_owned())] hashtags_base_url: String,
) -> impl IntoView {
    let i18n = use_i18n();

    let href = if website.is_published {
        Some(website.url.to_string())
    } else {
        None
    };

    let unpublished_tag = if !website.is_published {
        Either::Left(
            view! { <a class="btn btn-sm btn-outline btn-info no-animation">{t!(i18n, shared.unpublished)}</a> },
        )
    } else {
        Either::Right(())
    };

    view! {
        <div class="card card-sm bg-base-100 shadow-xl mb-4">
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

                <div class="empty:hidden my-1 flex gap-2 overflow-x-auto">
                    {unpublished_tag} <Hashtags hashtags=website.hashtags base_url=hashtags_base_url />
                </div>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
