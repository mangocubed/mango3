use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{
    ConfirmationDialog, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, PostCard,
};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::PlusOutlined;
use mango3_leptos_utils::models::PostPreviewResp;

use crate::context::param_website_id;
use crate::server_functions::{get_my_posts, AttemptToDeletePost};

#[component]
pub fn PostsPage() -> impl IntoView {
    let params_map = use_params_map();
    let i18n = use_i18n();
    let controller = InfiniteScrollResourceController::new(move |after| {
        Resource::new_blocking(
            move || (param_website_id(params_map).unwrap_or_default(), after.get()),
            |(website_id, after)| async { get_my_posts(website_id, after).await },
        )
    });
    let server_action = ServerAction::<AttemptToDeletePost>::new();
    let delete_post = RwSignal::new(None);
    let show_delete_confirmation = RwSignal::new(false);

    view! {
        <ConfirmationDialog
            is_open=show_delete_confirmation
            on_accept={
                let controller = controller.clone();
                move || {
                    let id = delete_post.get().map(|p: PostPreviewResp| p.id).unwrap();
                    server_action
                        .dispatch(AttemptToDeletePost {
                            website_id: param_website_id(params_map).unwrap_or_default(),
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
        </ConfirmationDialog>

        <h2 class="h2">{t!(i18n, shared.posts)}</h2>

        <section class="flex justify-end max-w-[720px] w-full mb-5 mx-auto">
            <a
                class="btn btn-outline"
                href=move || format!("/websites/{}/posts/new", param_website_id(params_map).unwrap_or_default())
            >
                <PlusOutlined />
                {t!(i18n, studio.new_post)}
            </a>
        </section>

        <section class="max-w-[720px] w-full mx-auto">
            <InfiniteScroll
                controller=controller
                key=|post: &PostPreviewResp| post.id.clone()
                children=move |post| {
                    view! {
                        <PostCard
                            post=post.clone()
                            hashtags_base_url=post.website.url.clone()
                            actions=move || {
                                let post = post.clone();
                                view! {
                                    <a
                                        class="btn btn-ghost font-bold"
                                        href={
                                            let post_id = post.id.clone();
                                            move || {
                                                format!(
                                                    "/websites/{}/posts/{}/edit",
                                                    param_website_id(params_map).unwrap_or_default(),
                                                    post_id,
                                                )
                                            }
                                        }
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
