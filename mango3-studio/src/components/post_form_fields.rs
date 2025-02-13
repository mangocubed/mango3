use leptos::either::Either;
use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use mango3_leptos_utils::utils::ToSignalTrait;
use server_fn::error::NoCustomError;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{
    ImageUploadField, MultipleImageUploadField, SubmitButton, SwitchField, TextField, TextareaField,
};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;

use crate::components::PostPreviewModal;
use crate::models::EditPostResp;

#[component]
pub fn PostFormFields(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(into)] website_id: TextProp,
    #[prop(optional)] post: Option<EditPostResp>,
) -> impl IntoView {
    let i18n = use_i18n();
    let error_title = RwSignal::new(None);
    let error_slug = RwSignal::new(None);
    let error_content = RwSignal::new(None);
    let error_variables = RwSignal::new(None);
    let error_publish = RwSignal::new(None);
    let value_title = RwSignal::new(post.as_ref().map(|p| p.title.clone()).unwrap_or_default());
    let value_slug = RwSignal::new(post.as_ref().map(|p| p.slug.clone()).unwrap_or_default());
    let value_content = RwSignal::new(post.as_ref().map(|p| p.content.clone()).unwrap_or_default());
    let value_variables = RwSignal::new(
        post.as_ref()
            .map(|p| p.variables.clone())
            .unwrap_or_else(|| "{}".to_owned()),
    );
    let value_blobs = RwSignal::new(post.as_ref().map(|p| p.blobs.clone()).unwrap_or_default());
    let value_cover_image_blob = RwSignal::new(post.as_ref().and_then(|p| p.cover_image_blob.clone()));
    let value_publish = post.map(|p| p.is_published).unwrap_or_default();
    let show_variables = RwSignal::new(false);
    let show_preview = RwSignal::new(false);

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

    let on_click_preview = move |event: MouseEvent| {
        event.prevent_default();

        show_preview.set(true);
    };

    view! {
        <input
            type="hidden"
            name="website_id"
            value={
                let website_id = website_id.clone();
                move || website_id.get()
            }
        />

        <TextField
            label=async_t_string!(i18n, studio.title).to_signal()
            name="title"
            error=error_title
            on_input=title_on_input
            value=value_title
        />

        <TextField label=async_t_string!(i18n, studio.slug).to_signal() name="slug" value=value_slug error=error_slug />

        <TextareaField
            label=async_t_string!(i18n, studio.content).to_signal()
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
            label=async_t_string!(i18n, studio.attached_images).to_signal()
            name="blob_ids"
            value=value_blobs
            website_id=website_id.clone()
        />

        <ImageUploadField
            label=async_t_string!(i18n, studio.cover_image).to_signal()
            id="cover_image_blob_id"
            name="cover_image_blob_id"
            width=288
            value=value_cover_image_blob
            website_id=website_id
        />

        <SwitchField
            label=async_t_string!(i18n, studio.publish).to_signal()
            name="publish"
            error=error_publish
            is_checked=value_publish
        />

        <div class="flex gap-2">
            <div class="py-2 w-full">
                <button class="btn btn-block btn-secondary btn-outline" on:click=on_click_preview>
                    {move || {
                        if is_loading.get() {
                            Either::Left(view! { <span class="loading loading-spinner" /> })
                        } else {
                            Either::Right(t!(i18n, studio.preview))
                        }
                    }}
                </button>
            </div>

            <SubmitButton is_loading=is_loading />
        </div>

        <PostPreviewModal
            is_active=show_preview
            title=value_title
            content=value_content
            variables=value_variables
            cover_image_blob=value_cover_image_blob
        />
    }
}
