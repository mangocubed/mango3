use leptos::ev::Event;
use leptos::prelude::*;
use server_fn::error::NoCustomError;

use mango3_leptos_utils::components::{ImageUploadField, SwitchField, TextField, TextareaField};
use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::models::{ActionFormResp, BlobResp};

use crate::models::EditPostResp;

#[component]
pub fn PostFormFields(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(optional)] post: Option<EditPostResp>,
) -> impl IntoView {
    let i18n = use_i18n();
    let error_title = RwSignal::new(None);
    let error_slug = RwSignal::new(None);
    let error_content = RwSignal::new(None);
    let error_publish = RwSignal::new(None);
    let value_title = post.as_ref().map(|p| p.title.clone()).unwrap_or_default();
    let value_slug = RwSignal::new(post.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let value_content = post.as_ref().map(|p| p.content.clone()).unwrap_or_default();
    let value_blobs = RwSignal::new(
        post.as_ref()
            .map(|p| p.attachments.to_vec())
            .unwrap_or_default()
            .iter()
            .map(|a| a.blob.clone())
            .collect::<Vec<BlobResp>>(),
    );
    let value_uploaded_blob = RwSignal::new(None);
    let value_cover_image_blob = RwSignal::new(post.as_ref().and_then(|p| p.cover_image_blob.clone()));
    let value_publish = post.map(|p| p.is_published).unwrap_or_default();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_title.set(response.error("title"));
        error_slug.set(response.error("slug"));
        error_content.set(response.error("content"));
    });

    Effect::new(move || {
        if let Some(blob) = value_uploaded_blob.get() {
            value_blobs.update(|blobs| {
                blobs.insert(0, blob);
            });
            value_uploaded_blob.set(None);
        }
    });

    let title_on_input = move |event: Event| {
        value_slug.set(slug::slugify(event_target_value(&event)));
    };

    view! {
        <TextField
            label=move || t_string!(i18n, studio.title)
            name="title"
            error=error_title
            on_input=title_on_input
            value=value_title
        />

        <TextField label=move || t_string!(i18n, studio.slug) name="slug" value=value_slug error=error_slug />

        <TextareaField
            label=move || t_string!(i18n, studio.content)
            name="content"
            rows=8
            value=value_content
            error=error_content
        />

        <ImageUploadField
            id="blob_ids"
            label=t_string!(i18n, studio.attached_images)
            name="blob_ids[]"
            value=value_uploaded_blob
        />

        <div class="form-control">
            <ForEnumerate
                each=move || value_blobs.get()
                key=|blob| blob.id.clone()
                children=move |index, blob| {
                    let value_blob = RwSignal::new(Some(blob));
                    Effect::new(move || {
                        if value_blob.get().is_none() {
                            value_blobs
                                .update(|blobs| {
                                    blobs.remove(index.get());
                                });
                        }
                    });
                    view! {
                        <ImageUploadField
                            id=index.with(|i| format!("blob_ids_{}", i))
                            name="blob_ids[]"
                            value=value_blob
                        />
                    }
                }
            />
        </div>

        <ImageUploadField
            label=move || t_string!(i18n, studio.cover_image)
            id="cover_image_blob_id"
            name="cover_image_blob_id"
            width=288
            value=value_cover_image_blob
        />

        <SwitchField
            label=move || t_string!(i18n, studio.publish)
            name="publish"
            error=error_publish
            is_checked=value_publish
        />
    }
}
