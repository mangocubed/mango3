use leptos::ev::Event;
use leptos::prelude::*;
use server_fn::error::NoCustomError;

use mango3_leptos_utils::components::{
    ImageUploadField, MultipleImageUploadField, SubmitButton, SwitchField, TextField, TextareaField,
};
use mango3_leptos_utils::i18n::{t, t_string, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;

use crate::models::EditPostResp;

#[component]
pub fn PostFormFields(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(optional)] post: Option<EditPostResp>,
) -> impl IntoView {
    let i18n = use_i18n();
    let error_title = RwSignal::new(None);
    let error_slug = RwSignal::new(None);
    let error_content = RwSignal::new(None);
    let error_variables = RwSignal::new(None);
    let error_publish = RwSignal::new(None);
    let value_title = post.as_ref().map(|p| p.title.clone()).unwrap_or_default();
    let value_slug = RwSignal::new(post.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let value_content = post.as_ref().map(|p| p.content.clone()).unwrap_or_default();
    let value_variables = post
        .as_ref()
        .map(|p| p.variables.clone())
        .unwrap_or_else(|| "{}".to_owned());
    let value_blobs = RwSignal::new(post.as_ref().map(|p| p.blobs.clone()).unwrap_or_default());
    let value_cover_image_blob = RwSignal::new(post.as_ref().and_then(|p| p.cover_image_blob.clone()));
    let value_publish = post.map(|p| p.is_published).unwrap_or_default();
    let show_variables = RwSignal::new(false);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_title.set(response.error("title"));
        error_slug.set(response.error("slug"));
        error_content.set(response.error("content"));
        error_variables.set(response.error("variables"));
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

        <div class="collapse collapse-arrow" class:collapse-open=move || show_variables.get()>
            <div
                class="collapse-title cursor-pointer"
                on:click=move |_| {
                    show_variables
                        .update(|value| {
                            *value = !*value;
                        })
                }
            >
                {t!(i18n, studio.variables)}
            </div>
            <div class="collapse-content">
                <TextareaField name="variables" rows=4 value=value_variables error=error_variables />
            </div>
        </div>

        <MultipleImageUploadField
            id="blob_ids"
            label=t_string!(i18n, studio.attached_images)
            name="blob_ids"
            value=value_blobs
        />

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

        <SubmitButton is_loading=is_loading />
    }
}
