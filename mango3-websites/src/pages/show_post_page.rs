use leptos::either::EitherOf3;
use leptos::prelude::*;

use mango3_leptos_utils::components::LoadingSpinner;
use mango3_leptos_utils::pages::NotFoundPage;
use mango3_leptos_utils::pages::Page;

use crate::context::use_slug_param;
use crate::server_functions::get_post;

#[component]
pub fn ShowPostPage() -> impl IntoView {
    let slug = use_slug_param();
    let post_resource = Resource::new_blocking(move || slug.get(), get_post);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match post_resource.get() {
                    Some(Ok(Some(post))) => {
                        EitherOf3::A(
                            view! {
                                <Page class="max-w-[1200px] w-full ml-auto mr-auto" title=post.title.clone()>
                                    <div class="card card-compact bg-base-100 shadow-xl mb-4">
                                        {
                                            let post_title = post.title.clone();
                                            move || {
                                                post.cover_image_blob
                                                    .clone()
                                                    .map(|cover_image_blob| {
                                                        view! {
                                                            <figure>
                                                                <img
                                                                    src=cover_image_blob.variant_url(1200, 200, true)
                                                                    alt=post_title.clone()
                                                                />
                                                            </figure>
                                                        }
                                                    })
                                            }
                                        } <div class="card-body">
                                            <h1 class="card-title h1 mb-6">{post.title}</h1>

                                            <div class="prose max-w-none" inner_html=post.content_html />
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
