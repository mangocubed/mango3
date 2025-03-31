use leptos::prelude::*;

use mango3_web_utils::components::{
    ConfirmationModal, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController, PostCard,
};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::PlusOutlined;
use mango3_web_utils::models::PostPreviewResp;

use crate::components::MyWebsitePageWrapper;
use crate::server_functions::{get_my_posts, AttemptToDeletePost};

#[component]
pub fn PostsPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <MyWebsitePageWrapper children=move |website| {
            let website_id = website.id.clone();
            let controller = InfiniteScrollLocalResourceController::new(|after| {
                LocalResource::new({
                    let website_id = website_id.clone();
                    move || get_my_posts(website_id.clone(), after.get())
                })
            });
            let server_action = ServerAction::<AttemptToDeletePost>::new();
            let delete_post = RwSignal::new(None);
            let show_delete_confirmation = RwSignal::new(false);

            view! {
                <ConfirmationModal
                    is_open=show_delete_confirmation
                    on_accept={
                        let controller = controller.clone();
                        let website_id = website_id.clone();
                        move || {
                            let id = delete_post.get().map(|p: PostPreviewResp| p.id).unwrap();
                            server_action
                                .dispatch(AttemptToDeletePost {
                                    website_id: website_id.clone(),
                                    id: id.clone(),
                                });
                            controller
                                .nodes
                                .update(|p| {
                                    p.retain(|p: &PostPreviewResp| p.id != id);
                                });
                            delete_post.set(None);
                        }
                    }
                >
                    {t!(i18n, studio.are_you_sure_you_want_to_delete_this_post)}
                </ConfirmationModal>

                <h2 class="h2">{t!(i18n, shared.posts)}</h2>

                <section class="flex justify-end max-w-[720px] w-full mb-5 mx-auto">
                    <a class="btn btn-outline" href=format!("/websites/{}/posts/new", website_id.clone())>

                        <PlusOutlined />
                        {t!(i18n, studio.new_post)}
                    </a>
                </section>

                <section class="max-w-[720px] w-full mx-auto">
                    <InfiniteScroll
                        controller=controller
                        key=|post: &PostPreviewResp| post.id.clone()
                        children=move |post| {
                            let website_id = website_id.clone();
                            let post_id = post.id.clone();
                            view! {
                                <PostCard
                                    post=post.clone()
                                    hashtags_base_url=post.website.url.clone()
                                    actions=move || {
                                        view! {
                                            <a
                                                class="btn btn-ghost font-bold"
                                                href=format!("/websites/{}/posts/{}/edit", &website_id, &post_id)
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
        } />
    }
}
