use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_web_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, LoadingSpinner, PostCard,
};
use mango3_web_utils::context::param_name;
use mango3_web_utils::pages::{NotFoundPage, Page};
use mango3_web_utils::presenters::PostMinPresenter;

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
                        let controller = InfiniteScrollResourceController::new(|after| {
                            Resource::new_blocking(
                                { move || (hashtag.id, after.get()) },
                                move |(hashtag_id, after)| async move { get_hashtag_posts(hashtag_id, after).await },
                            )
                        });
                        EitherOf3::A(
                            view! {
                                <Page title=format!("#{}", hashtag.name)>
                                    <h1 class="h1">"#"{hashtag.name}</h1>

                                    <section class="max-w-[720px] w-full mx-auto">
                                        <InfiniteScroll
                                            controller=controller
                                            key=|post: &PostMinPresenter| post.id
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
