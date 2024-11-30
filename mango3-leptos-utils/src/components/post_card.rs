use leptos::prelude::*;

use crate::models::PostResp;

#[component]
pub fn PostCard(
    post: PostResp,
    #[prop(into, default = false)] show_content: bool,
    #[prop(into, optional)] actions: ViewFn,
) -> impl IntoView {
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
                <h3 class="card-title">
                    <a href=move || {
                        if !show_content && post.is_published { Some(post.url.clone()) } else { None }
                    }>{post.title}</a>
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
