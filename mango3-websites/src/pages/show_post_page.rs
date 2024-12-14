use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_meta::Meta;

use mango3_leptos_utils::components::{LoadingSpinner, TimeAgo, UserTag};
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::components::MetaDateTime;
use crate::context::use_slug_param;
use crate::server_functions::get_post;

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let i18n = use_i18n();
    let slug = use_slug_param();
    let post_resource = Resource::new_blocking(move || slug.get(), get_post);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match post_resource.get() {
                    Some(Ok(Some(post))) => {
                        EitherOf3::A(
                            view! {
                                <Meta name="description" content=post.title.clone() />
                                <Meta name="author" content=post.user.display_name.clone() />
                                <Meta property="article:author:username" content=post.user.username.clone() />

                                <Page class="max-w-[1200px] w-full ml-auto mr-auto" title=post.title.clone()>
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
                                        post.updated_at
                                            .map(|updated_at| {
                                                view! {
                                                    <MetaDateTime property="article:modified_time" content=updated_at />
                                                }
                                            })
                                    }}

                                    <div class="card card-compact bg-base-100 shadow-xl">
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
                                                </div>
                                            </div>

                                            <div class="prose max-w-none break-words" inner_html=post.content_html />

                                            <div class="mt-4 opacity-70">
                                                {move || {
                                                    if post.views_count == 1 {
                                                        t_string!(i18n, shared.one_view).to_owned()
                                                    } else {
                                                        t_string!(i18n, shared.count_views, count = post.views_count)
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    </div>
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
