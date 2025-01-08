use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{InfiniteScroll, PostCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::components::CurrentWebsiteOpt;
use crate::server_functions::get_posts;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let posts_resource =
        Resource::new_blocking(move || after.get(), |after| async move { get_posts(None, after).await });

    view! {
        <CurrentWebsiteOpt children=move |website| {
            match website {
                Some(website) => {
                    Either::Left(
                        view! {
                            <Page title=move || t_string!(i18n, shared.home)>
                                <section class="max-w-[1200px] w-full mx-auto">
                                    {move || {
                                        website
                                            .cover_image_blob
                                            .clone()
                                            .map(|blob| {
                                                view! {
                                                    <img class="rounded mb-4" src=blob.variant_url(1200, 200, true) />
                                                }
                                            })
                                    }} <h3 class="text-lg font-bold">{website.description}</h3>

                                </section>

                                <section class="max-w-[640px] w-full mx-auto mt-4">
                                    <InfiniteScroll
                                        after=after
                                        key=|post: &PostPreviewResp| post.id.clone()
                                        resource=posts_resource
                                        let:post
                                    >
                                        <PostCard post=post />
                                    </InfiniteScroll>
                                </section>
                            </Page>
                        },
                    )
                }
                None => Either::Right(NotFoundPage),
            }
        } />
    }
}
