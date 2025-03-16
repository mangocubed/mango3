use leptos::prelude::*;

use mango3_leptos_utils::components::forms::MultipleImageUploadField;
use mango3_leptos_utils::components::{
    ConfirmationModal, CopyableText, InfiniteScroll, InfiniteScrollControllerTrait,
    InfiniteScrollLocalResourceController,
};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::BlobResp;

use crate::components::MyWebsitePageWrapper;
use crate::server_functions::{get_my_blobs, AttemptToDeleteBlob};

#[component]
pub fn FilesPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToDeleteBlob>::new();
    let delete_blob = RwSignal::new(None);
    let show_delete_confirmation = RwSignal::new(false);
    let uploaded_files = RwSignal::new(vec![]);

    view! {
        <MyWebsitePageWrapper children=move |website| {
            let website_id = website.id.clone();
            let controller = InfiniteScrollLocalResourceController::new(|after| {
                LocalResource::new({
                    let website_id = website_id.clone();
                    move || get_my_blobs(website_id.clone(), after.get())
                })
            });
            Effect::new({
                let controller = controller.clone();
                move || {
                    if !uploaded_files.get().is_empty() {
                        controller.clear_and_refetch();
                        uploaded_files.set(vec![]);
                    }
                }
            });

            view! {
                <h2 class="h2">{t!(i18n, studio.files)}</h2>

                <ConfirmationModal
                    is_open=show_delete_confirmation
                    on_accept={
                        let controller = controller.clone();
                        let website_id = website_id.clone();
                        move || {
                            let id = delete_blob.get().map(|b: BlobResp| b.id).unwrap();
                            server_action
                                .dispatch(AttemptToDeleteBlob {
                                    website_id: website_id.clone(),
                                    id: id.clone(),
                                });
                            controller
                                .nodes
                                .update(|blobs| {
                                    blobs.retain(|b: &BlobResp| b.id != id);
                                });
                            delete_blob.set(None);
                        }
                    }
                >
                    {t!(i18n, studio.are_you_sure_you_want_to_delete_this_file)}
                </ConfirmationModal>

                <section class="flex max-w-[720px] w-full mb-5 mx-auto">
                    <MultipleImageUploadField
                        label=move || t!(i18n, studio.upload_files)
                        website_id=website_id.clone()
                        value=uploaded_files
                    />
                </section>

                <section class="max-w-[720px] w-full mx-auto">
                    <InfiniteScroll
                        controller=controller
                        key=|blob: &BlobResp| blob.id.clone()
                        children=move |blob| {
                            view! {
                                <div class="card card-sm bg-base-200 shadow-xl mb-4">
                                    <div class="card-body">
                                        <div class="flex flex-row gap-3 items-center">
                                            <div class="avatar">
                                                <div class="rounded" style:width="64px" style:height="64px">
                                                    <img
                                                        alt=blob.file_name.clone()
                                                        src=blob.variant_url(64, 64, true)
                                                    />
                                                </div>
                                            </div>

                                            <div class="flex flex-col gap-4 w-full">
                                                <div class="card-title">
                                                    <div class="font-bold">{blob.file_name.clone()}</div>
                                                </div>

                                                <CopyableText value=blob.url.clone() />

                                                <Show when=move || {
                                                    blob.is_removable
                                                }>
                                                    {
                                                        let blob = blob.clone();
                                                        move || {
                                                            let blob = blob.clone();
                                                            view! {
                                                                <div class="card-actions justify-end">
                                                                    <button
                                                                        class="btn btn-ghost font-bold"
                                                                        on:click=move |_| {
                                                                            delete_blob.set(Some(blob.clone()));
                                                                            show_delete_confirmation.set(true);
                                                                        }
                                                                    >
                                                                        {t!(i18n, studio.delete)}
                                                                    </button>
                                                                </div>
                                                            }
                                                        }
                                                    }
                                                </Show>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </section>
            }
        } />
    }
}
