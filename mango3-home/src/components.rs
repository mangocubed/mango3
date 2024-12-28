use leptos::prelude::*;

use leptos_router::params::ParamsMap;
use mango3_leptos_utils::components::{InfiniteScroll, PostCard, WebsiteCard};
use mango3_leptos_utils::context::param_query;
use mango3_leptos_utils::models::{PostPreviewResp, WebsiteResp};

use crate::server_functions::{get_posts_search, get_websites_search};

#[component]
pub fn SearchPostsTab(params_map: Memo<ParamsMap>) -> impl IntoView {
    let after = RwSignal::new(None);
    let is_loading = RwSignal::new(true);
    let nodes = RwSignal::new(Vec::new());
    let posts_resource = Resource::new_blocking(
        move || (param_query(params_map), after.get()),
        |(query, after)| async { get_posts_search(query, after).await },
    );

    Effect::new(move || {
        param_query(params_map);
        nodes.set(vec![]);
        is_loading.set(true);
    });

    view! {
        <InfiniteScroll
            after=after
            key=|post: &PostPreviewResp| post.id.clone()
            is_loading=is_loading
            nodes=nodes
            resource=posts_resource
            let:post
        >
            <PostCard post=post show_host=true />
        </InfiniteScroll>
    }
}

#[component]
pub fn SearchWebsitesTab(params_map: Memo<ParamsMap>) -> impl IntoView {
    let after = RwSignal::new(None);
    let is_loading = RwSignal::new(true);
    let nodes = RwSignal::new(Vec::new());
    let websites_resource = Resource::new_blocking(
        move || (param_query(params_map), after.get()),
        |(query, after)| async { get_websites_search(query, after).await },
    );

    view! {
        <InfiniteScroll
            after=after
            key=|website: &WebsiteResp| website.id.clone()
            is_loading=is_loading
            nodes=nodes
            resource=websites_resource
            let:website
        >
            <WebsiteCard website=website />
        </InfiniteScroll>
    }
}
