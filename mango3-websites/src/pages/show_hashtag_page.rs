use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{InfiniteScroll, PostCard};
use mango3_leptos_utils::context::param_name;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::server_functions::{get_hashtag, get_posts};

#[component]
pub fn ShowHashtagPage() -> impl IntoView {
    let params_map = use_params_map();
    let hashtag_resource = Resource::new_blocking(move || param_name(params_map), get_hashtag);

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                match hashtag_resource.get() {
                    Some(Ok(Some(hashtag))) => {
                        let after = RwSignal::new(None);
                        let posts_resource = Resource::new_blocking(
                            move || (param_name(params_map), after.get()),
                            |(name, after)| async move { get_posts(Some(name), after).await },
                        );
                        let title = move || format!("#{}", hashtag.name);
                        EitherOf3::A(
                            view! {
                                <Page title=title.clone()>
                                    <h1 class="h2">{title}</h1>

                                    <section class="max-w-[640px] w-full mx-auto">
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
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(()),
                }
            })}
        </Suspense>
    }
}
