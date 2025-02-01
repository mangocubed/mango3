use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, LoadingSpinner, PostCard};
use mango3_leptos_utils::context::param_name;
use mango3_leptos_utils::models::{CursorPageResp, PostPreviewResp};
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::server_functions::{get_hashtag, get_posts};

#[component]
pub fn ShowHashtagPage() -> impl IntoView {
    let params_map = use_params_map();
    let hashtag_resource = Resource::new_blocking(move || param_name(params_map), get_hashtag);
    let controller = InfiniteScrollController::new(move |after| {
        Resource::new_blocking(
            move || {
                (
                    hashtag_resource.with(|resource| {
                        resource
                            .as_ref()
                            .and_then(|result| result.as_ref().ok())
                            .flatten()
                            .map(|hashtag| hashtag.name.clone())
                    }),
                    after.get(),
                )
            },
            move |(hashtag_name, after)| async move {
                if let Some(name) = hashtag_name {
                    get_posts(Some(name), after).await
                } else {
                    Ok(CursorPageResp::default())
                }
            },
        )
    });

    Effect::new({
        let controller = controller.clone();
        move || {
            hashtag_resource.track();

            controller.clear_and_refetch();
        }
    });

    view! {
        <Transition fallback=LoadingSpinner>
            {move || Suspend::new({
                let controller = controller.clone();
                async move {
                    match hashtag_resource.get() {
                        Some(Ok(Some(hashtag))) => {
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
                }
            })}
        </Transition>
    }
}
