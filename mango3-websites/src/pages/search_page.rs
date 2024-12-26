use leptos::prelude::*;

use leptos_router::hooks::use_query_map;
use mango3_leptos_utils::components::{InfiniteScroll, PostCard};
use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_posts_search;
use mango3_leptos_utils::context::param_query;

#[component]
pub fn SearchPage() -> impl IntoView {
    let i18n = use_i18n();
    let query_map = use_query_map();
    let after = RwSignal::new(None);
    let is_loading = RwSignal::new(true);
    let nodes = RwSignal::new(Vec::new());
    let posts_resource = Resource::new_blocking(
        move || (param_query(query_map), after.get()),
        |(query, after)| async { get_posts_search(query, after).await },
    );

    Effect::new(move || {
        param_query(query_map);
        nodes.set(vec![]);
        is_loading.set(true);
    });

    let title = move || t_string!(i18n, shared.search_results_for, query = param_query(query_map));

    view! {
        <Page title=title>
            <h1 class="h2">{title}</h1>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll
                    after=after
                    key=|post: &PostPreviewResp| post.id.clone()
                    is_loading=is_loading
                    nodes=nodes
                    resource=posts_resource
                    let:post
                >
                    <PostCard post=post />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
