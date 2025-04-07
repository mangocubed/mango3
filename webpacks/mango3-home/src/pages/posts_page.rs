use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, PostCard,
};
use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::pages::Page;
use mango3_web_utils::presenters::PostMinPresenter;
use mango3_web_utils::utils::ToSignalTrait;

use crate::server_functions::get_posts;

#[component]
pub fn PostsPage() -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollResourceController::new(|after| {
        Resource::new_blocking(move || after.get(), |after| async move { get_posts(10, after).await })
    });
    let text_title = async_t_string!(i18n, shared.posts).to_signal();

    view! {
        <Page title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <section class="max-w-[720px] w-full ml-auto mr-auto">
                <InfiniteScroll controller=controller key=|post: &PostMinPresenter| post.id let:post>
                    <PostCard post=post show_host=true />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
