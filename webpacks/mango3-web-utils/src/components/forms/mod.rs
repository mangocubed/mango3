use leptos::ev::Event;
use leptos::prelude::*;

use crate::components::Modal;
use crate::presenters::{MutPresenter, MutPresenterActionValue};

mod country_field;
mod password_field;
mod submit_button;
mod switch_field;
mod text_field;
mod textarea_field;

#[cfg(feature = "image-upload")]
mod image_upload_field;
#[cfg(feature = "markdown-editor")]
mod markdown_editor_field;
#[cfg(feature = "multiple-image-upload")]
mod multiple_image_upload_field;

pub use country_field::CountryField;
pub use password_field::PasswordField;
pub use submit_button::SubmitButton;
pub use switch_field::SwitchField;
pub use text_field::TextField;
pub use textarea_field::TextareaField;

#[cfg(feature = "image-upload")]
pub use image_upload_field::ImageUploadField;
#[cfg(feature = "markdown-editor")]
pub use markdown_editor_field::MarkdownEditorField;
#[cfg(feature = "multiple-image-upload")]
pub use multiple_image_upload_field::MultipleImageUploadField;

pub struct EventFn(Box<dyn Fn(Event) + Send + Sync + 'static>);

impl<T> From<T> for EventFn
where
    T: Fn(Event) + Send + Sync + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

#[component]
fn FieldError(error: RwSignal<Option<String>>) -> impl IntoView {
    view! { <div class="fieldset-label text-error">{move || error.get()}</div> }
}

#[component]
fn FieldLabel(id: String, children: Children) -> impl IntoView {
    view! {
        <label class="fieldset-label empty:hidden" for=id>
            {children()}
        </label>
    }
}

#[component]
pub fn FormErrorAlert<D>(
    #[prop(optional)] action_value: MutPresenterActionValue<D>,
    #[prop(into, optional)] is_active: RwSignal<bool>,
    #[prop(into)] message: ViewFn,
) -> impl IntoView
where
    D: Clone + Default + Send + Sync + 'static,
{
    Effect::new(move || {
        let response = MutPresenter::from(action_value);

        is_active.set(response.is_invalid());
    });

    view! {
        <Show when=move || is_active.get()>
            <div class="py-2 has-[div:empty]:hidden">
                <div role="alert" class="alert alert-error">
                    {message.run()}
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn FormField<D>(
    #[prop(optional)] action_value: MutPresenterActionValue<D>,
    children: Children,
    #[prop(optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: String,
    #[prop(into)] label: ViewFn,
    #[prop(into, optional)] name: String,
) -> impl IntoView
where
    D: Clone + Default + Send + Sync + 'static,
{
    Effect::new(move || {
        let response = MutPresenter::from(action_value);
        let name = name.clone();

        if !name.is_empty() {
            error.set(response.error(name));
        }
    });

    view! {
        <fieldset class="fieldset w-full">
            <FieldLabel id=id>{label.run()}</FieldLabel>

            {children()}

            <FieldError error=error />
        </fieldset>
    }
}

#[component]
pub fn FormSuccessModal(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] is_open: RwSignal<bool>,
    #[prop(into)] message: ViewFn,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    Effect::new(move || {
        let response = MutPresenter::from(action_value);

        is_open.set(response.is_success());
    });

    view! {
        <Modal is_closable=false is_open=is_open>
            <div>{message.run()}</div>
            <div class="modal-action">
                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                        if let Some(oc) = on_close {
                            oc.run(())
                        }
                    }
                >
                    "Ok"
                </button>
            </div>
        </Modal>
    }
}
