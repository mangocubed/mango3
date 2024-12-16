use leptos::prelude::*;

use crate::components::UserTag;
use crate::i18n::{t, t_string, use_i18n};
use crate::models::PostPreviewResp;

use super::TimeAgo;

#[component]
pub fn PostCard(
    post: PostPreviewResp,
    #[prop(into, optional)] actions: Option<ViewFnOnce>,
    #[prop(optional)] show_host: bool,
) -> impl IntoView {
    let i18n = use_i18n();

    let href = move || {
        if post.is_published {
            Some(post.url.clone())
        } else {
            None
        }
    };

    view! {
        <div class="card card-compact bg-base-200 shadow-xl mb-4">
            {
                let post_title = post.title.clone();
                let href = href.clone();
                move || {
                    post.cover_image_blob
                        .clone()
                        .map(|cover_image_blob| {
                            view! {
                                <figure>
                                    <a href=href.clone() title=post_title.clone()>
                                        <img src=cover_image_blob.variant_url(1200, 200, true) alt=post_title.clone() />
                                    </a>
                                </figure>
                            }
                        })
                }
            } <div class="card-body">
                <h3 class="card-title">
                    <a href=href.clone()>{post.title}</a>
                </h3>

                <div class="flex justify-between my-1">
                    <UserTag user=post.user />

                    <div class="text-right opacity-70">
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

                        <Show when=move || show_host>
                            <div class="text-right opacity-70">{post.host.clone()}</div>
                        </Show>
                    </div>
                </div>

                <a href=href class="card-text-preview break-words">
                    <div class="prose max-w-none" inner_html=post.content_preview_html />
                    <div class="card-text-preview-overlay" />
                </a>

                <div class="my-1 opacity-70">
                    {move || {
                        if post.views_count == 1 {
                            t_string!(i18n, shared.one_view).to_owned()
                        } else {
                            t_string!(i18n, shared.count_views, count = post.views_count)
                        }
                    }}
                </div>

                {actions.map(|a| view! { <div class="card-actions justify-end">{a.run()}</div> })}
            </div>
        </div>
    }
}
