use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{Hashtags, LoadingSpinner, PostBottomBar, TimeAgo, UserTag};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::components::{HighLightCode, MetaDateTime, PostComments, PostReactions};
use crate::context::param_slug;
use crate::server_functions::get_post;

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let post_resource = Resource::new_blocking(move || param_slug(params_map), get_post);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match post_resource.get() {
                    Some(Ok(Some(post))) => {
                        EitherOf3::A(
                            view! {
                                <Page class="max-w-[1200px] w-full ml-auto mr-auto" title=post.title.clone()>
                                    <Meta name="description" content=post.title.clone() />
                                    <Meta name="author" content=post.user.display_name.clone() />
                                    <Meta property="article:author:username" content=post.user.username.clone() />

                                    {move || {
                                        post.published_at
                                            .map(|published_at| {
                                                view! {
                                                    <MetaDateTime
                                                        property="article:published_time"
                                                        content=published_at
                                                    />
                                                }
                                            })
                                    }}

                                    {move || {
                                        post.modified_at
                                            .map(|modified_at| {
                                                view! {
                                                    <MetaDateTime
                                                        property="article:modified_time"
                                                        content=modified_at
                                                    />
                                                }
                                            })
                                    }}

                                    <div class="card card-compact bg-base-200 shadow-xl">
                                        {
                                            let post_title = post.title.clone();
                                            move || {
                                                post.cover_image_blob
                                                    .clone()
                                                    .map(|cover_image_blob| {
                                                        let cover_image_url = cover_image_blob
                                                            .variant_url(1200, 200, true);
                                                        view! {
                                                            <Meta property="og:image" content=cover_image_url.clone() />
                                                            <figure>
                                                                <img src=cover_image_url alt=post_title.clone() />
                                                            </figure>
                                                        }
                                                    })
                                            }
                                        } <div class="card-body">
                                            <h1 class="card-title h1 mb-6">{post.title}</h1>

                                            <div class="flex justify-between my-4">
                                                <a
                                                    href=post.user.url
                                                    title={
                                                        let username = post.user.username;
                                                        move || format!("@{}", username)
                                                    }
                                                >
                                                    <UserTag user=post.user.clone() />
                                                </a>

                                                <div class="text-right opacity-70">
                                                    <TimeAgo value=post.created_at />

                                                    {move || {
                                                        post.modified_at
                                                            .map(|modified_ad| {
                                                                view! {
                                                                    " ("
                                                                    {t!(i18n, shared.edited)}
                                                                    " "
                                                                    <TimeAgo value=modified_ad />
                                                                    ")"
                                                                }
                                                            })
                                                    }}
                                                </div>
                                            </div>

                                            <div
                                                class="prose prose-pre:bg-transparent max-w-none break-words"
                                                inner_html=post.content_html.clone()
                                            />

                                            <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                                <Hashtags hashtags=post.hashtags />
                                            </div>

                                            <PostBottomBar
                                                comments_count=post.comments_count
                                                reactions_count=post.reactions_count
                                                views_count=post.views_count
                                            />

                                            <PostReactions post_id=post.id.clone() />

                                            <PostComments post_id=post.id />
                                        </div>
                                    </div>

                                    <HighLightCode content=post.content_html />
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(view! { <div /> }),
                }
            })}
        </Suspense>
    }
}
