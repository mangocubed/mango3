use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, LoadingSpinner, PostCard};
use mango3_leptos_utils::context::param_name;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::server_functions::{get_hashtag, get_hashtag_posts};

#[component]
pub fn ShowHashtagPage() -> impl IntoView {
    let params_map = use_params_map();
    let hashtag_resource = Resource::new_blocking(move || param_name(params_map), get_hashtag);

    view! {
        <Transition fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match hashtag_resource.get() {
                    Some(Ok(Some(hashtag))) => {
                        let controller = InfiniteScrollController::new(|after| {
                            Resource::new_blocking(
                                {
                                    let hashtag_id = hashtag.id.clone();
                                    move || (hashtag_id.clone(), after.get())
                                },
                                move |(hashtag_id, after)| async move { get_hashtag_posts(hashtag_id, after).await },
                            )
                        });
                        let title = move || format!("#{}", hashtag.name);
                        EitherOf3::A(
                            view! {
                                <Page title=title.clone()>
                                    <h1 class="h2">{title}</h1>

                                    <section class="max-w-[720px] w-full mx-auto">
                                        <InfiniteScroll
                                            controller=controller
                                            key=|post: &PostPreviewResp| post.id.clone()
                                            let:post
                                        >
                                            <PostCard post=post />
                                        </InfiniteScroll>
                                    </section>
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(()),
                }
            })}
        </Transition>
    }
}
