use leptos::prelude::*;

use mango3_leptos_utils::components::{ConfirmationDialog, InfiniteScroll, PostCard};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::PostResp;

use crate::context::use_website_id_param;
use crate::server_functions::{get_my_posts, AttemptToDeletePost};

#[component]
pub fn PostsPage() -> impl IntoView {
    let website_id = use_website_id_param();
    let i18n = use_i18n();
    let after = RwSignal::new(None);
    let website_id_clone = website_id.clone();
    let my_posts_resource = Resource::new_blocking(
        move || (website_id_clone.clone(), after.get()),
        |(website_id, after)| async { get_my_posts(website_id, after).await },
    );
    let posts = RwSignal::new(vec![]);
    let server_action = ServerAction::<AttemptToDeletePost>::new();
    let delete_post = RwSignal::new(None);
    let show_delete_confirmation = RwSignal::new(false);

    view! {
        <ConfirmationDialog
            is_open=show_delete_confirmation
            on_accept={
                let website_id = website_id.clone();
                move || {
                    let id = delete_post.get().map(|p: PostResp| p.id).unwrap();
                    server_action
                        .dispatch(AttemptToDeletePost {
                            website_id: website_id.clone(),
                            id: id.clone(),
                        });
                    posts
                        .update(|p| {
                            p.retain(|p: &PostResp| p.id != id);
                        });
                    delete_post.set(None);
                }
            }
        >
            {t!(i18n, studio.are_you_sure_you_want_to_delete_this_post)}
        </ConfirmationDialog>

        <h3 class="h3">{t!(i18n, shared.posts)}</h3>

        <section class="max-w-[640px] w-full ml-auto mr-auto">
            <InfiniteScroll
                after=after
                key=|post: &PostResp| post.id.clone()
                resource=my_posts_resource
                nodes=posts
                children=move |post| {
                    let website_id = website_id.clone();
                    let post = post.clone();
                    view! {
                        <PostCard
                            post=post.clone()
                            actions=move || {
                                let post = post.clone();
                                view! {
                                    <a
                                        class="btn btn-ghost font-bold"
                                        href=format!("/websites/{}/posts/{}/edit", &website_id, &post.id)
                                    >
                                        {t!(i18n, studio.edit)}
                                    </a>

                                    <button
                                        class="btn btn-ghost font-bold"
                                        on:click=move |_| {
                                            delete_post.set(Some(post.clone()));
                                            show_delete_confirmation.set(true);
                                        }
                                    >
                                        {t!(i18n, studio.delete)}
                                    </button>
                                }
                            }
                        />
                    }
                }
            />
        </section>
    }
}