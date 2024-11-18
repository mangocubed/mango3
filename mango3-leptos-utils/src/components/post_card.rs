use leptos::prelude::*;

use crate::i18n::{t, use_i18n};
use crate::models::PostResp;

#[component]
pub fn PostCard(
    post: PostResp,
    #[prop(into, default = false)] show_content: bool,
    #[prop(into, optional)] actions: ViewFn,
) -> impl IntoView {
    let i18n = use_i18n();
    let inner_html = move || {
        if show_content {
            post.content_html.clone()
        } else {
            post.content_preview_html.clone()
        }
    };

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
                <h3 class="card-title">{post.title}</h3>

                <div class="prose" inner_html=inner_html />

                <div class="card-actions justify-end">
                    <Show when=move || !show_content && post.is_published>
                        <a class="btn btn-ghost font-bold" href=post.url.clone()>
                            {t!(i18n, shared.view_post)}
                        </a>
                    </Show>

                    {actions.run()}
                </div>
            </div>
        </div>
    }
}
