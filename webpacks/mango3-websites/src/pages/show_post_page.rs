use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_router::hooks::use_params_map;

use mango3_web_utils::components::{Hashtags, LoadingSpinner, Modal, PostBottomBar, UserTagLink};
use mango3_web_utils::pages::NotFoundPage;
use mango3_web_utils::pages::Page;

use crate::components::{HighLightCode, MetaDateTime, PostComments, PostReactions};
use crate::context::param_slug;
use crate::server_functions::get_post;

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let params_map = use_params_map();
    let post_resource = Resource::new_blocking(move || param_slug(params_map), get_post);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match post_resource.get() {
                    Some(Ok(Some(post))) => {
                        let image_modal_url = RwSignal::new(None);
                        let image_modal_is_open = RwSignal::new(false);
                        Effect::new(move || {
                            image_modal_is_open.set(image_modal_url.get().is_some());
                        });
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
                                                            <Meta
                                                                property="og:image"
                                                                content=cover_image_url.to_string()
                                                            />
                                                            <figure>
                                                                <img
                                                                    src=cover_image_url.to_string()
                                                                    alt=post_title.clone()
                                                                />
                                                            </figure>
                                                        }
                                                    })
                                            }
                                        } <div class="card-body">
                                            <h1 class="card-title h1 text-2xl">{post.title}</h1>

                                            <div class="my-4">
                                                <UserTagLink user=post.user />
                                            </div>

                                            <div
                                                class="prose prose-pre:bg-transparent prose-img:mx-auto max-w-none break-words"
                                                inner_html=post.content_html.clone()
                                            />

                                            <div class="empty:hidden flex flex-wrap gap-3 my-4">
                                                <For each=move || post.blobs.clone() key=|blob| blob.id let:blob>
                                                    <figure
                                                        class="rounded"
                                                        on:click=move |_| {
                                                            image_modal_url.set(Some(blob.url.clone()));
                                                        }
                                                    >
                                                        <img src=blob.variant_url(128, 128, true).to_string() />
                                                    </figure>
                                                </For>
                                            </div>

                                            <Modal
                                                class="overflow-y-visible"
                                                box_class="overflow-y-visible max-w-[max-content]"
                                                is_open=image_modal_is_open
                                            >
                                                <figure class="max-w-full">
                                                    <img
                                                        class="max-w-[calc(100vw-120px)] max-h-[calc(100vh-120px)]"
                                                        src=move || image_modal_url.get().map(|url| url.to_string())
                                                    />
                                                </figure>
                                            </Modal>

                                            <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                                <Hashtags hashtags=post.hashtags />
                                            </div>

                                            <PostBottomBar
                                                comments_count=post.comments_count
                                                reactions_count=post.reactions_count
                                                views_count=post.views_count
                                                created_at=post.created_at
                                                modified_at=post.modified_at
                                            />

                                            <PostReactions post_id=post.id />

                                            <PostComments post_id=post.id />
                                        </div>
                                    </div>

                                    <HighLightCode content=post.content_html />
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => {
                        EitherOf3::C(

                            view! { <div /> },
                        )
                    }
                }
            })}
        </Suspense>
    }
}
