use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::{Hashtags, LoadingSpinner, UserTagLink};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::BlobResp;

use crate::components::{HighLightCode, MyWebsite};
use crate::server_functions::preview_post;

#[component]
pub fn PostPreviewModal(
    is_active: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] content: Signal<String>,
    #[prop(into)] variables: Signal<String>,
    #[prop(into)] cover_image_blob: Signal<Option<BlobResp>>,
) -> impl IntoView {
    let i18n = use_i18n();
    let preview_action = Action::new(
        move |(title, content, variables, cover_image_blob_id): &(String, String, String, Option<String>)| {
            let title = title.to_owned();
            let content = content.to_owned();
            let variables = variables.to_owned();
            let cover_image_blob_id = cover_image_blob_id.to_owned();

            async move { preview_post(title, content, variables, cover_image_blob_id).await }
        },
    );
    let preview_action_value = preview_action.value();

    Effect::new(move || {
        if is_active.get() {
            preview_action.dispatch((
                title.get(),
                content.get(),
                variables.get(),
                cover_image_blob.get().map(|blob| blob.id),
            ));
        }
    });

    view! {
        <Show when=move || {
            is_active.get()
        }>
            {move || {
                view! {
                    <div class="modal modal-open p-8 overflow-y-auto">
                        <div class="modal-box max-w-[1200px] w-[calc(100vw-64px)] max-h-[unset] overflow-y-visible">
                            <div class="card card-compact bg-base-200 shadow-xl">
                                {move || {
                                    if let Some(Ok(post)) = preview_action_value.get() {
                                        Either::Left(
                                            view! {
                                                {
                                                    let post_title = post.title.clone();
                                                    move || {
                                                        post.cover_image_blob
                                                            .clone()
                                                            .map(|cover_image_blob| {
                                                                let cover_image_url = cover_image_blob
                                                                    .variant_url(1200, 200, true);
                                                                view! {
                                                                    <figure>
                                                                        <img src=cover_image_url alt=post_title.clone() />
                                                                    </figure>
                                                                }
                                                            })
                                                    }
                                                }

                                                <div class="card-body">
                                                    <h1 class="card-title h1 mb-6">{post.title}</h1>

                                                    <div class="my-4">
                                                        <UserTagLink user=post.user />
                                                    </div>

                                                    <div
                                                        class="prose prose-pre:bg-transparent prose-img:mx-auto max-w-none break-words"
                                                        inner_html=post.content_html.clone()
                                                    />

                                                    <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                                        <MyWebsite let:website>
                                                            <Hashtags
                                                                hashtags=post.hashtags.clone()
                                                                base_url=website.url
                                                            />
                                                        </MyWebsite>
                                                    </div>
                                                </div>

                                                <HighLightCode content=post.content_html />
                                            },
                                        )
                                    } else {
                                        Either::Right(LoadingSpinner)
                                    }
                                }}
                            </div>

                            <div class="flex justify-end mt-4">
                                <button on:click=move |_| is_active.set(false) class="btn btn-outline">
                                    {t!(i18n, studio.close_preview)}
                                </button>
                            </div>
                        </div>

                        <div class="modal-backdrop" on:click=move |_| is_active.set(false) />
                    </div>
                }
            }}
        </Show>
    }
}
