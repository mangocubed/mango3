use leptos::either::Either;
use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::components::{Hashtags, LoadingSpinner, Modal, UserTagLink};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::presenters::BlobPresenter;

use crate::components::{HighLightCode, MyWebsite};
use crate::server_functions::preview_post;

#[component]
pub fn PostPreviewModal(
    is_open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] content: Signal<String>,
    #[prop(into)] variables: Signal<String>,
    #[prop(into)] cover_image_blob: Signal<Option<BlobPresenter>>,
) -> impl IntoView {
    let i18n = use_i18n();
    let preview_action = Action::new(
        move |(title, content, variables, cover_image_blob_id): &(String, String, String, Option<Uuid>)| {
            let title = title.to_owned();
            let content = content.to_owned();
            let variables = variables.to_owned();
            let cover_image_blob_id = cover_image_blob_id.to_owned();

            async move { preview_post(title, content, variables, cover_image_blob_id).await }
        },
    );
    let preview_action_value = preview_action.value();

    Effect::new(move || {
        if is_open.get() {
            preview_action.dispatch((
                title.get(),
                content.get(),
                variables.get(),
                cover_image_blob.get().map(|blob| blob.id),
            ));
        }
    });

    view! {
        <Modal
            is_open=is_open
            class="p-8 overflow-y-auto"
            box_class="max-w-[1200px] w-[calc(100vw-64px)] max-h-[unset] overflow-y-visible"
        >
            <div class="card card-sm bg-base-200 shadow-xl">
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
                                                view! {
                                                    <figure>
                                                        <img
                                                            src=cover_image_blob
                                                                .variant_url(1200, 200, true)
                                                                .to_string()
                                                            alt=post_title.clone()
                                                        />
                                                    </figure>
                                                }
                                            })
                                    }
                                }

                                <div class="card-body">
                                    <h2 class="card-title h1 text-2xl">{post.title}</h2>

                                    <div class="my-4">
                                        <UserTagLink user=post.user />
                                    </div>

                                    <div
                                        class="prose prose-pre:bg-transparent prose-img:mx-auto max-w-none break-words"
                                        inner_html=post.content_html.clone()
                                    />

                                    <div class="empty:hidden my-4 flex flex-wrap gap-2">
                                        <MyWebsite let:website>
                                            <Hashtags hashtags=post.hashtags.clone() base_url=website.url.to_string() />
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
                <button
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false)
                    }
                    class="btn btn-outline"
                >
                    {t!(i18n, studio.close_preview)}
                </button>
            </div>
        </Modal>
    }
}
