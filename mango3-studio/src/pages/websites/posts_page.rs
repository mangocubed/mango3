use leptos::prelude::*;

use mango3_leptos_utils::components::{InfiniteScroll, PostCard};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::PostResp;

use crate::components::MyWebsiteResource;
use crate::server_functions::get_my_posts;

#[component]
pub fn PostsPage() -> impl IntoView {
    view! {
        <MyWebsiteResource children=move |website| {
            website
                .map(|website| {
                    let i18n = use_i18n();
                    let after = RwSignal::new(None);
                    let website_id = website.id.clone();
                    let my_posts_resource = Resource::new_blocking(
                        move || (website.id.clone(), after.get()),
                        |(website_id, after)| async { get_my_posts(website_id, after).await },
                    );
                    view! {
                        <section class="max-w-[640px] w-full ml-auto mr-auto">
                            <InfiniteScroll
                                after=after
                                key=|post: &PostResp| post.id.clone()
                                resource=my_posts_resource
                                children=move |post| {
                                    let website_id = website_id.clone();
                                    view! {
                                        <PostCard
                                            post=post.clone()
                                            actions=move || {
                                                view! {
                                                    <a
                                                        class="btn btn-ghost font-bold"
                                                        href=format!("/websites/{}/posts/{}/edit", website_id, post.id)
                                                    >
                                                        {t!(i18n, studio.edit)}
                                                    </a>
                                                }
                                            }
                                        />
                                    }
                                }
                            />
                        </section>
                    }
                })
        } />
    }
}
