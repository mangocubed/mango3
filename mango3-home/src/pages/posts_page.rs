use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, PostCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_posts;

#[component]
pub fn PostsPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollController::new(|after| {
        Resource::new_blocking(move || after.get(), |after| async move { get_posts(10, after).await })
    });

    let title = move || t_string!(i18n, shared.posts);

    view! {
        <Page title=title>
            <h2 class="h2">{title}</h2>

            <section class="max-w-[720px] w-full ml-auto mr-auto">
                <InfiniteScroll controller=controller key=|post: &PostPreviewResp| post.id.clone() let:post>
                    <PostCard post=post show_host=true />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
