use leptos::either::EitherOf3;
use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlInputElement};

use crate::components::{CopyableText, LoadingSpinner};
use crate::icons::TrashOutlined;
use crate::models::BlobResp;
use crate::server_functions::attempt_to_upload_file;

#[component]
pub fn ImageUploadField(
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(default = 48)] height: i16,
    #[prop(into)] id: String,
    #[prop(into, optional)] label: TextProp,
    #[prop(into, optional)] value: RwSignal<Option<BlobResp>>,
    #[prop(default = 48)] width: i16,
    #[prop(into)] name: String,
) -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| attempt_to_upload_file(data.clone().into()));
    let upload_action_value = upload_action.value();

    Effect::new(move || {
        if let Some(Ok(blob_resp)) = upload_action_value.get() {
            value.set(blob_resp)
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

        let _ = form_data.append_with_blob_and_filename("file", &file, &file.name());

        upload_action.dispatch_local(form_data);
    };

    let remove = move |event: MouseEvent| {
        event.prevent_default();

        value.set(None)
    };

    view! {
        <div class="form-control w-full">
            <label class="label" for=id.clone()>
                <span class="label-text">{move || label.get()}</span>
            </label>
            {move || {
                if upload_action.pending().get() {
                    EitherOf3::A(LoadingSpinner)
                } else if let Some(blob) = value.get() {
                    let variant_url = blob.variant_url(width, height, true).clone();
                    EitherOf3::B(
                        view! {
                            <input type="hidden" name=name.clone() value=blob.id />
                            <div class="flex flex-wrap gap-3">
                                <img class="rounded" width=width height=height src=variant_url />

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
                                class="file-input file-input-bordered w-full"
                                class:file-input-error=has_error
                                id=id.clone()
                                type="file"
                                accept="image/bmp,image/gif,image/jpeg,image/png"
                                on:change=upload
                            />
                        },
                    )
                }
            }}
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
