use leptos::either::Either;
use leptos::prelude::*;

use leptos_router::hooks::use_query_map;
use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, PostCard};
use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::components::CurrentWebsiteOpt;
use crate::server_functions::get_posts_search;
use mango3_leptos_utils::context::param_query;

#[component]
pub fn SearchPage() -> impl IntoView {
    view! {
        <CurrentWebsiteOpt children=move |website| {
            match website {
                Some(_) => {
                    let i18n = use_i18n();
                    let query_map = use_query_map();
                    let controller = InfiniteScrollController::new(|after| Resource::new_blocking(
                        move || (param_query(query_map), after.get()),
                        |(query, after)| async { get_posts_search(query, after).await },
                    ));
                    Effect::new({
                        let controller = controller.clone();
                        move || {
                            query_map.track();
                            controller.clear_and_refetch();
                        }
                    });
                    let title = move || t_string!(i18n, shared.search_results_for, query = param_query(query_map));
                    Either::Left(
                        view! {
                            <Page title=title>
                                <h1 class="h2">{title}</h1>

                                <section class="max-w-[640px] w-full mx-auto">
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
                None => Either::Right(NotFoundPage),
            }
        } />
    }
}
