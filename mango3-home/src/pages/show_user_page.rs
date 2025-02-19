use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{
    Hashtags, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController, LoadingSpinner, PostCard,
};
use mango3_leptos_utils::models::{CursorPageResp, PostPreviewResp};
use mango3_leptos_utils::pages::{NotFoundPage, Page};

use crate::context::param_username;
use crate::server_functions::{get_user, get_user_posts};

#[component]
pub fn ShowUserPage() -> impl IntoView {
    let params_map = use_params_map();
    let user_resource = Resource::new_blocking(move || param_username(params_map), get_user);

    view! {
        <Transition fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match user_resource.get() {
                    Some(Ok(Some(user))) => {
                        let controller = InfiniteScrollResourceController::new(move |after| {
                            Resource::new_blocking(
                                move || after.get(),
                                move |after| async move {
                                    let user_id = user_resource.await.ok().flatten().map(|u| u.id);
                                    if let Some(user_id) = user_id {
                                        get_user_posts(user_id, after).await
                                    } else {
                                        Ok(CursorPageResp::default())
                                    }
                                },
                            )
                        });
                        let avatar_image_url = user.avatar_image_url(256);
                        EitherOf3::A(
                            view! {
                                <Page title=format!("{} (@{})", user.display_name, user.username)>
                                    <div class="flex flex-wrap justify-center gap-6 max-w-[1200px] mx-auto">
                                        <div class="card card-sm bg-base-200 shadow-xl flex-1 self-start min-w-[320px] max-w-[640px]">
                                            <div class="card-body">
                                                <div class="avatar self-center mt-4">
                                                    <div class="rounded-full w-64 h-64">
                                                        <img
                                                            alt=user.initials.clone()
                                                            class="rounded-full"
                                                            src=avatar_image_url
                                                        />
                                                    </div>
                                                </div>

                                                <h1 class="h1 mb-1 text-center">{user.display_name}</h1>

                                                <h2 class="h2 opacity-70 font-normal text-center">
                                                    "@"{user.username}
                                                </h2>

                                                <div
                                                    class="prose prose-pre:bg-transparent max-w-none break-words"
                                                    inner_html=user.bio_html.clone()
                                                />

                                                <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                                    <Hashtags hashtags=user.hashtags />
                                                </div>
                                            </div>
                                        </div>

                                        <div class="shrink-0 max-w-[720px] w-full">
                                            <InfiniteScroll
                                                controller=controller
                                                key=|post: &PostPreviewResp| post.id.clone()
                                                let:post
                                            >
                                                <PostCard post=post show_host=true />
                                            </InfiniteScroll>
                                        </div>
                                    </div>
                                </Page>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(NotFoundPage),
                    _ => EitherOf3::C(()),
                }
            })}
        </Transition>
    }
}
