use leptos::prelude::*;

use crate::components::{Hashtags, UserIcon};
use crate::i18n::{t, use_i18n};
use crate::models::UserPreviewResp;

#[component]
pub fn UserCard(
    #[prop(into)] user: UserPreviewResp,
    #[prop(into, optional)] actions: ViewFn,
    #[prop(default = "/".to_owned())] hashtags_base_url: String,
) -> impl IntoView {
    let i18n = use_i18n();

    let user_url = user.url.clone();

    let href = if !user.is_disabled {
        Some(user_url.clone())
    } else {
        None
    };

    view! {
        <div class="card card-sm bg-base-100 shadow-xl mb-4">
            <div class="card-body">
                <div class="flex gap-3 items-center">
                    <a href=href.clone()>
                        <UserIcon user=user.clone() />
                    </a>

                    <div class="card-title">
                        <a href=href.clone()>
                            <div class="font-bold text-lg">{user.display_name.clone()}</div>
                            <div class="text-base opacity-70">"@"{user.username.clone()}</div>
                        </a>
                    </div>
                </div>

                <a href=href class="card-text-preview">
                    <div class="prose max-w-none break-words" inner_html=user.bio_preview_html.clone() />
                    <div class="card-text-preview-overlay to-base-100" />
                </a>

                <div class="empty:hidden my-1 flex gap-2 overflow-x-auto">
                    <Show when={
                        let is_disabled = user.is_disabled;
                        move || is_disabled
                    }>
                        <a class="btn btn-sm btn-outline btn-error no-animation">{t!(i18n, shared.disabled)}</a>
                    </Show>

                    <Hashtags hashtags=user.hashtags.clone() base_url=hashtags_base_url />
                </div>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
