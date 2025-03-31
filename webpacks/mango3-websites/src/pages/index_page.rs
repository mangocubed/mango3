use leptos::either::Either;
use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{
    Hashtags, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, PostCard,
};
use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::models::PostPreviewResp;
use mango3_web_utils::pages::{NotFoundPage, Page};
use mango3_web_utils::utils::ToSignalTrait;

use crate::components::CurrentWebsiteOpt;
use crate::server_functions::get_posts;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollResourceController::new(|after| {
        Resource::new_blocking(move || after.get(), |after| async move { get_posts(None, after).await })
    });
    let text_title = async_t_string!(i18n, shared.home).to_signal();

    view! {
        <CurrentWebsiteOpt children=move |website| {
            match website {
                Some(website) => {
                    let controller = controller.clone();
                    Either::Left(
                        view! {
                            <Page title=text_title>
                                <section class="max-w-[1200px] w-full mx-auto">
                                    {move || {
                                        website
                                            .cover_image_blob
                                            .clone()
                                            .map(|blob| {
                                                view! { <img class="rounded" src=blob.variant_url(1200, 200, true) /> }
                                            })
                                    }}
                                </section>

                                <section class="flex flex-wrap justify-center gap-6 max-w-[1200px] mt-4 mx-auto">
                                    <div class="card card-compact bg-base-200 shadow-xl flex-1 self-start min-w-[320px] max-w-[640px]">
                                        <div class="card-body">
                                            <div
                                                class="prose prose-pre:bg-transparent max-w-none break-words"
                                                inner_html=website.description_html.clone()
                                            />

                                            <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                                <Hashtags hashtags=website.hashtags />
                                            </div>
                                        </div>
                                    </div>

                                    <div class="shrink-0 max-w-[720px] w-full">
                                        <InfiniteScroll
                                            controller=controller
                                            key=|post: &PostPreviewResp| post.id.clone()
                                            let:post
                                        >
                                            <PostCard post=post />
                                        </InfiniteScroll>
                                    </div>
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
