use leptos::either::Either;
use leptos::ev::{Event, MouseEvent};
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use server_fn::error::NoCustomError;

use mango3_web_utils::components::forms::{
    ImageUploadField, MarkdownEditorField, MultipleImageUploadField, SubmitButton, SwitchField, TextField,
    TextareaField,
};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::presenters::MutPresenter;

use crate::components::PostPreviewModal;
use crate::presenters::EditPostPresenter;

#[component]
pub fn PostFormFields(
    action_value: RwSignal<Option<Result<MutPresenter, ServerFnError<NoCustomError>>>>,
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(into)] website_id: TextProp,
    #[prop(optional)] post: Option<EditPostPresenter>,
) -> impl IntoView {
    let i18n = use_i18n();
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
            action_value=action_value
            id="title"
            label=move || t!(i18n, studio.title)
            name="title"
            on_input=title_on_input
            value=value_title
        />

        <TextField
            action_value=action_value
            id="slug"
            label=move || t!(i18n, studio.slug)
            name="slug"
            value=value_slug
        />

        <MarkdownEditorField
            action_value=action_value
            id="content"
            label=move || t!(i18n, studio.content)
            name="content"
            rows=8
            value=value_content
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
                <TextareaField action_value=action_value id="variables" name="variables" rows=4 value=value_variables />
            </div>
        </div>

        <MultipleImageUploadField
            id="blob_ids"
            label=move || t!(i18n, studio.attached_images)
            name="blob_ids"
            value=value_blobs
            website_id=website_id.clone()
        />

        <ImageUploadField
            action_value=action_value
            id="cover_image_blob_id"
            label=move || t!(i18n, studio.cover_image)
            name="cover_image_blob_id"
            width=288
            value=value_cover_image_blob
            website_id=website_id
        />

        <SwitchField
            action_value=action_value
            id="publish"
            label=move || t!(i18n, studio.publish)
            name="publish"
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
            is_open=show_preview
            title=value_title
            content=value_content
            variables=value_variables
            cover_image_blob=value_cover_image_blob
        />
    }
}
