use leptos::prelude::*;

use leptos_router::params::ParamsMap;
use mango3_web_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, PostCard, WebsiteCard,
};
use mango3_web_utils::context::param_query;
use mango3_web_utils::presenters::{PostMinPresenter, WebsiteMinPresenter};

use crate::server_functions::{get_posts_search, get_websites_search};

#[component]
pub fn SearchPostsTab(params_map: Memo<ParamsMap>) -> impl IntoView {
    let controller = InfiniteScrollResourceController::new(move |after| {
        Resource::new_blocking(
            move || (param_query(params_map), after.get()),
            |(query, after)| async move { get_posts_search(query, after).await },
        )
    });

    Effect::new({
        let controller = controller.clone();
        move || {
            param_query(params_map);
            controller.clear_and_refetch();
        }
    });

    view! {
        <InfiniteScroll controller=controller key=|post: &PostMinPresenter| post.id let:post>
            <PostCard post=post show_host=true />
        </InfiniteScroll>
    }
}

#[component]
pub fn SearchWebsitesTab(params_map: Memo<ParamsMap>) -> impl IntoView {
    let controller = InfiniteScrollResourceController::new(move |after| {
        Resource::new_blocking(
            move || (param_query(params_map), after.get()),
            |(query, after)| async move { get_websites_search(query, after).await },
        )
    });

    view! {
        <InfiniteScroll controller=controller key=|website: &WebsiteMinPresenter| website.id let:website>
            <WebsiteCard website=website />
        </InfiniteScroll>
    }
}
