use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{InfiniteScroll, InfiniteScrollController, PostCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::Page;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::server_functions::get_posts;

#[component]
pub fn PostsPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollController::new(|after| {
        Resource::new_blocking(move || after.get(), |after| async move { get_posts(10, after).await })
    });
    let text_title = async_t_string!(i18n, shared.posts).to_signal();

    view! {
        <Page title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <section class="max-w-[720px] w-full ml-auto mr-auto">
                <InfiniteScroll controller=controller key=|post: &PostPreviewResp| post.id.clone() let:post>
                    <PostCard post=post show_host=true />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
