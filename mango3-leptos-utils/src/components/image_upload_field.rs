use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos::{either::EitherOf3, ev::Event};
use leptos_fluent::tr;
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlInputElement};

use crate::models::BlobResp;

#[server(input = MultipartFormData)]
pub async fn attempt_to_upload_file(data: MultipartData) -> Result<Option<BlobResp>, ServerFnError> {
    use crate::ssr::{expect_core_context, extract_user, require_authentication};

    use mango3_core::models::Blob;

    if !require_authentication().await? {
        return Ok(None);
    }

    let Some(mut data) = data.into_inner() else {
        return Ok(None);
    };

    let Some(mut field) = data.next_field().await? else {
        return Ok(None);
    };

    let Some("file") = field.name() else {
        return Ok(None);
    };

    let core_context = expect_core_context();
    let user = extract_user().await?.unwrap();

    let blob = Blob::insert(&core_context, &user, &mut field).await.ok();

    Ok(blob.map(|blob| blob.into()))
}

#[component]
pub fn ImageUploadField(
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(default = 48)] height: i16,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: TextProp,
    #[prop(default = 48)] width: i16,
    name: &'static str,
) -> impl IntoView {
    let upload_action = Action::new_local(|data: &FormData| attempt_to_upload_file(data.clone().into()));
    let upload_action_value = upload_action.value();
    let value = RwSignal::new(None);

    Effect::new(move || {
        if let Some(Ok(blob_resp)) = upload_action_value.get() {
            value.set(blob_resp)
        }
    });

    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

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

    view! {
        <div class="form-control w-full">
            <label class="label" for=field_id>
                <span class="label-text">{move || label.get()}</span>
            </label>
            {move || {
                if upload_action.pending().get() {
                    EitherOf3::A(view! { <span class="loading loading-spinner loading-lg"></span> })
                } else if let Some(blob) = value.get() {
                    let variant_url = blob.variant_url(width, height, true).clone();
                    EitherOf3::B(
                        view! {
                            <input type="hidden" name=name value=blob.id />
                            <div class="flex">
                                <img class="rounded" width=width height=height src=variant_url />
                                <button class="btn">{move || tr!("remove")}</button>
                            </div>
                        },
                    )
                } else {
                    EitherOf3::C(
                        view! {
                            <input
                                class="file-input file-input-bordered w-full"
                                class:file-input-error=has_error
                                id=field_id
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
