use leptos::prelude::*;

use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal, MarkdownEditorField, SubmitButton};
use mango3_web_utils::components::{
    CurrentUser, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController, TimeAgo,
    UserTag, UserTagLink,
};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::models::PostCommentResp;

use crate::server_functions::{get_post_comments, AttemptToCreatePostComment};

#[allow(unused_variables)]
#[component]
pub fn PostComments(post_id: String) -> impl IntoView {
    let i18n = use_i18n();
    let controller = InfiniteScrollLocalResourceController::new(|after| {
        let post_id = post_id.clone();
        LocalResource::new(move || {
            let post_id = post_id.clone();
            async move { get_post_comments(post_id.clone(), after.get()).await }
        })
    });

    view! {
        <section>
            <h2 class="h2 mt-5">{t!(i18n, websites.comments)}</h2>

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
                                    let server_action = ServerAction::<AttemptToCreatePostComment>::new();
                                    let action_value = server_action.value();
                                    let value_content = RwSignal::new(String::new());
                                    view! {
                                        <div class="flex gap-4 items-start">
                                            <UserTag class="my-4" text_class="hidden md:block" user=user />

                                            <ActionForm
                                                action=server_action
                                                attr:autocomplete="off"
                                                attr:novalidate="true"
                                                attr:class="form max-w-full"
                                            >
                                                <FormErrorAlert
                                                    action_value=action_value
                                                    message=move || { t!(i18n, websites.failed_to_submit_comment) }
                                                />

                                                <input type="hidden" name="post_id" value=post_id />

                                                <MarkdownEditorField
                                                    action_value=action_value
                                                    id="content"
                                                    name="content"
                                                    rows=2
                                                    value=value_content
                                                />

                                                <SubmitButton is_loading=server_action.pending() />
                                            </ActionForm>

                                            <FormSuccessModal
                                                message=move || { t!(i18n, websites.comment_submitted_successfully) }
                                                on_close=move || {
                                                    action_value.set(None);
                                                    value_content.set(String::new());
                                                    controller.clear_and_refetch();
                                                }
                                            />
                                        </div>
                                    }
                                }
                            } />

                            <InfiniteScroll
                                controller=controller
                                key=|comment: &PostCommentResp| comment.id.clone()
                                let:post_comment
                            >
                                <div class="card card-sm card-border border-neutral-500 mt-4">
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
        </section>
    }
}
