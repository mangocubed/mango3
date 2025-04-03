use leptos::either::Either;
use leptos::prelude::*;

use leptos_router::hooks::use_query_map;
use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, PostCard,
};
use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::pages::{NotFoundPage, Page};
use mango3_web_utils::presenters::PostMinPresenter;

use crate::components::CurrentWebsiteOpt;
use crate::server_functions::get_posts_search;
use mango3_web_utils::context::param_query;

#[component]
pub fn SearchPage() -> impl IntoView {
    view! {
        <CurrentWebsiteOpt children=move |website| {
            match website {
                Some(_) => {
                    let i18n = use_i18n();
                    let query_map = use_query_map();
                    let controller = InfiniteScrollResourceController::new(|after| Resource::new_blocking(
                        move || (param_query(query_map), after.get()),
                        |(query, after)| async move { get_posts_search(query, after).await },
                    ));
                    Effect::new({
                        let controller = controller.clone();
                        move || {
                            query_map.track();
                            controller.clear_and_refetch();
                        }
                    });
                    let text_title = Signal::derive(move || {
                        async_t_string!(i18n, shared.search_results_for, query = param_query(query_map))
                            .with(|value| value.clone().unwrap_or("Search results".to_owned()))
                    });
                    Either::Left(
                        view! {
                            <Page title=text_title>
                                <h1 class="h2">{move || text_title.get()}</h1>

                                <section class="max-w-[720px] w-full mx-auto">
                                    <InfiniteScroll
                                        controller=controller
                                        key=|post: &PostMinPresenter| post.id.clone()
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
