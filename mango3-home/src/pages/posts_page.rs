use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{InfiniteScroll, PostCard};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::PostPreviewResp;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::get_posts;

#[component]
pub fn PostsPage() -> impl IntoView {
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let posts_resource = Resource::new_blocking(move || after.get(), |after| async move { get_posts(10, after).await });

    let title = move || t_string!(i18n, shared.posts);

    view! {
        <Page title=title>
            <h2 class="h2">{title}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <InfiniteScroll
                    after=after
                    key=|post: &PostPreviewResp| post.id.clone()
                    resource=posts_resource
                    let:post
                >
                    <PostCard post=post show_host=true />
                </InfiniteScroll>
            </section>
        </Page>
    }
}
