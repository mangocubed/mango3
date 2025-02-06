use leptos::prelude::*;

use mango3_leptos_utils::components::{
    ActionFormAlert, CurrentUser, InfiniteScroll, InfiniteScrollController, SubmitButton, TextareaField, TimeAgo,
    UserTag, UserTagLink,
};
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::models::{ActionFormResp, PostCommentResp};

use crate::server_functions::{get_post_comments, AttemptToCreatePostComment};

#[allow(unused_variables)]
#[component]
pub fn PostComments(post_id: String) -> impl IntoView {
    let i18n = use_i18n();
    let is_browser = RwSignal::new(false);

    Effect::new(move || {
        is_browser.set(true);
    });

    view! {
        <section>
            <h2 class="h2 mt-5">{t!(i18n, websites.comments)}</h2>

            <Show when=move || {
                is_browser.get()
            }>
                {
                    let post_id = post_id.clone();
                    move || {
                        let post_id = post_id.clone();
                        let controller = InfiniteScrollController::new(|after| Resource::new_blocking(
                            {
                                let post_id = post_id.clone();
                                move || (post_id.clone(), after.get())
                            },
                            |(post_id, after)| async move { get_post_comments(post_id, after).await },
                        ));
                        view! {
                            <Transition>
                                {move || Suspend::new({
                                    let post_id = post_id.clone();
                                    let controller = controller.clone();
                                    async move {
                                        view! {
                                            <CurrentUser children={
                                                let controller = controller.clone();
                                                move |user| {
                                                    let controller = controller.clone();
                                                    let post_id = post_id.clone();
                                                    let server_action = ServerAction::<
                                                        AttemptToCreatePostComment,
                                                    >::new();
                                                    let action_value = server_action.value();
                                                    let value_content = RwSignal::new(String::new());
                                                    let error_content = RwSignal::new(None);
                                                    Effect::new(move || {
                                                        let response = ActionFormResp::from(action_value);
                                                        error_content.set(response.error("content"));
                                                    });
                                                    view! {
                                                        <div class="flex gap-4 items-start">
                                                            <UserTag
                                                                class="my-4"
                                                                text_class="hidden md:block"
                                                                user=user
                                                            />

                                                            <ActionForm
                                                                action=server_action
                                                                attr:autocomplete="off"
                                                                attr:novalidate="true"
                                                                attr:class="form max-w-full"
                                                            >
                                                                <ActionFormAlert
                                                                    action_value=action_value
                                                                    error_message=move || {
                                                                        t_string!(i18n, websites.failed_to_submit_comment)
                                                                    }
                                                                    on_success=move || {
                                                                        action_value.set(None);
                                                                        value_content.set(String::new());
                                                                        controller.clear_and_refetch();
                                                                    }
                                                                    success_message=move || {
                                                                        t_string!(i18n, websites.comment_submitted_successfully)
                                                                    }
                                                                />
                                                                <input type="hidden" name="post_id" value=post_id />

                                                                <TextareaField
                                                                    name="content"
                                                                    rows=2
                                                                    error=error_content
                                                                    value=value_content
                                                                />

                                                                <SubmitButton is_loading=server_action.pending() />
                                                            </ActionForm>
                                                        </div>
                                                    }
                                                }
                                            } />

                                            <InfiniteScroll
                                                controller=controller
                                                key=|comment: &PostCommentResp| comment.id.clone()
                                                let:post_comment
                                            >
                                                <div class="card card-compact card-bordered border-neutral-500 mt-4">
                                                    <div class="card-body">
                                                        <div class="flex gap-4 items-start">
                                                            <UserTagLink user=post_comment.user />

                                                            <div class="flex-1">
                                                                <div class="text-right opacity-70">
                                                                    <TimeAgo value=post_comment.created_at />
                                                                </div>

                                                                <div
                                                                    class="prose prose-pre:bg-transparent max-w-none break-words"
                                                                    inner_html=post_comment.content_html.clone()
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </InfiniteScroll>
                                        }
                                    }
                                })}
                            </Transition>
                        }
                    }
                }
            </Show>
        </section>
    }
}
