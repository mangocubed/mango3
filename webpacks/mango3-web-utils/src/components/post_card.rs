use leptos::either::Either;
use leptos::prelude::*;

use crate::components::{Hashtags, WebsiteIcon};
use crate::i18n::{t, use_i18n};
use crate::presenters::PostMinPresenter;

use super::{PostBottomBar, UserTagLink};

#[component]
pub fn PostCard(
    #[prop(into)] post: PostMinPresenter,
    #[prop(into, optional)] actions: Option<ViewFnOnce>,
    #[prop(optional)] show_host: bool,
    #[prop(default = "/".to_owned())] hashtags_base_url: String,
) -> impl IntoView {
    let i18n = use_i18n();

    let href = if post.is_published {
        Some(post.url.to_string())
    } else {
        None
    };

    let unpublished_tag = if !post.is_published {
        Either::Left(
            view! { <a class="btn btn-sm btn-outline btn-info no-animation">{t!(i18n, shared.unpublished)}</a> },
        )
    } else {
        Either::Right(())
    };

    view! {
        <div class="card card-sm bg-base-200 shadow-xl mb-4">
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
                                        <img
                                            src=cover_image_blob.variant_url(1200, 200, true).to_string()
                                            alt=post_title.clone()
                                        />
                                    </a>
                                </figure>
                            }
                        })
                }
            } <div class="card-body">
                <h3 class="card-title text-xl">
                    <a href=href.clone()>{post.title}</a>
                </h3>

                <div class="flex justify-between my-1">
                    <UserTagLink user=post.user />

                    <Show when=move || show_host>
                        <div class="text-right">
                            {t!(
                                i18n,
                                shared.on_subdomain,
                                subdomain = {
                                    let website = post.website.clone();
                                    move || view! {
                                        <a class="font-bold ml-2" href=website.url.to_string() title=website.name.clone()>
                                            <WebsiteIcon class="align-middle mr-2" size=16 website=website.clone() />
                                            {website.host.clone()}
                                        </a>
                                    }
                                }
                            )}
                        </div>
                    </Show>
                </div>

                <a href=href class="card-text-preview">
                    <div class="prose max-w-none break-words" inner_html=post.content_preview_html />
                    <div class="card-text-preview-overlay to-base-200" />
                </a>

                <div class="empty:hidden my-1 flex gap-2 overflow-x-auto">
                    {unpublished_tag} <Hashtags hashtags=post.hashtags base_url=hashtags_base_url />
                </div>

                <PostBottomBar
                    comments_count=post.comments_count
                    reactions_count=post.reactions_count
                    views_count=post.views_count
                    created_at=post.created_at
                    modified_at=post.modified_at
                />

                {actions.map(|a| view! { <div class="card-actions justify-end">{a.run()}</div> })}
            </div>
        </div>
    }
}
