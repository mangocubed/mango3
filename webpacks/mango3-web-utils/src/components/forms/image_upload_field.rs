use leptos::either::EitherOf3;
use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlInputElement};

use crate::components::{CopyableText, LoadingSpinner};
use crate::icons::TrashOutlined;
use crate::presenters::{BlobPresenter, MutPresenterActionValue};
use crate::server_functions::attempt_to_upload_image;

use super::FormField;

#[component]
pub fn ImageUploadField(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(default = 48)] height: u16,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] value: RwSignal<Option<BlobPresenter>>,
    #[prop(default = 48)] width: u16,
    #[prop(into, optional)] website_id: TextProp,
) -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| attempt_to_upload_image(data.clone().into()));
    let upload_action_value = upload_action.value();
    let website_id_store = StoredValue::new(website_id);

    Effect::new(move || {
        if let Some(Ok(success)) = upload_action_value.get() {
            value.set(success.data)
        }
    });

    let has_error = move || error.get().is_some();

    let upload = move |event: Event| {
        let Some(file) = event
            .target()
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .files()
            .and_then(|files| files.get(0))
        else {
            return;
        };

        let Ok(form_data) = FormData::new() else {
            return;
        };

        let website_id = website_id_store.read_value().get();

        let _ = form_data.append_with_str("website_id", &website_id);

        let _ = form_data.append_with_blob_and_filename("file", &file, &file.name());

        upload_action.dispatch_local(form_data);
    };

    let remove = move |event: MouseEvent| {
        event.prevent_default();

        value.set(None)
    };

    view! {
        <FormField action_value=action_value error=error id=id.clone() label=label name=name.clone()>
            {move || {
                if upload_action.pending().get() {
                    EitherOf3::A(LoadingSpinner)
                } else if let Some(blob) = value.get() {
                    EitherOf3::B(
                        view! {
                            <input type="hidden" name=name.clone() value=blob.id.to_string() />
                            <div class="flex flex-wrap gap-3">
                                <img
                                    class="rounded"
                                    width=width
                                    height=height
                                    src=blob.variant_url(width, height, true).to_string()
                                />

                                <div class="flex flex-1 gap-3">
                                    <CopyableText value=blob.url />

                                    <button class="btn" on:click=remove>
                                        <TrashOutlined />
                                    </button>
                                </div>
                            </div>
                        },
                    )
                } else {
                    EitherOf3::C(
                        view! {
                            <input
                                class="file-input w-full"
                                class:file-input-error=has_error
                                id=id.clone()
                                type="file"
                                accept="image/bmp,image/gif,image/jpeg,image/png,image/webp"
                                on:change=upload
                            />
                        },
                    )
                }
            }}
        </FormField>
    }
}
