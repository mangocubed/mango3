use leptos::prelude::*;

use crate::i18n::{t, use_i18n};
use crate::models::PostPreviewResp;

use super::TimeAgo;

#[component]
pub fn PostCard(post: PostPreviewResp, #[prop(into, optional)] actions: ViewFn) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="card card-compact bg-base-100 shadow-xl mb-4">
            {
                let post_title = post.title.clone();
                move || {
                    post.cover_image_blob
                        .clone()
                        .map(|cover_image_blob| {
                            view! {
                                <figure>
                                    <img src=cover_image_blob.variant_url(1200, 200, true) alt=post_title.clone() />
                                </figure>
                            }
                        })
                }
            } <div class="card-body">
                <h3 class="card-title">
                    <a href=move || if post.is_published { Some(post.url.clone()) } else { None }>{post.title}</a>
                </h3>

                <div class="self-end text-right">
                    <TimeAgo value=post.created_at />

                    {move || {
                        post.updated_at
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

                <div class="card-text-preview">
                    <div class="prose max-w-none" inner_html=post.content_preview_html />
                    <div class="card-text-preview-overlay" />
                </div>

                <div class="card-actions justify-end">{actions.run()}</div>
            </div>
        </div>
    }
}
