use leptos::prelude::*;

use crate::components::Hashtags;
use crate::i18n::{t, use_i18n};
use crate::models::PostPreviewResp;

use super::{PostBottomBar, TimeAgo, UserTag};

#[component]
pub fn PostCard(
    #[prop(into)] post: PostPreviewResp,
    #[prop(into, optional)] actions: Option<ViewFnOnce>,
    #[prop(optional)] show_host: bool,
    #[prop(default = "/".to_owned())] hashtags_base_url: String,
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
                            post.modified_at
                                .map(|modified_at| {
                                    view! {
                                        " ("
                                        {t!(i18n, shared.edited)}
                                        " "
                                        <TimeAgo value=modified_at />
                                        ")"
                                    }
                                })
                        }}

                        <Show when=move || show_host>
                            <div class="text-right opacity-70">{post.website.host.clone()}</div>
                        </Show>
                    </div>
                </div>

                <a href=href class="card-text-preview">
                    <div class="prose max-w-none break-words" inner_html=post.content_preview_html />
                    <div class="card-text-preview-overlay to-base-200" />
                </a>

                <div class="empty:hidden my-1 flex gap-2 overflow-x-auto">
                    <Show when=move || !post.is_published>
                        <a class="btn btn-sm btn-outline btn-info no-animation">{t!(i18n, shared.unpublished)}</a>
                    </Show>

                    <Hashtags hashtags=post.hashtags base_url=hashtags_base_url />
                </div>

                <PostBottomBar
                    comments_count=post.comments_count
                    reactions_count=post.reactions_count
                    views_count=post.views_count
                />

                {actions.map(|a| view! { <div class="card-actions justify-end">{a.run()}</div> })}
            </div>
        </div>
    }
}
