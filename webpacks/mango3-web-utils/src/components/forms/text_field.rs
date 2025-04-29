#[cfg(feature = "with-dioxus")]
use dioxus::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos::ev::keydown;
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos_use::use_event_listener;

#[cfg(feature = "with-dioxus")]
use crate::presenters::MutPresenter;

#[cfg(not(feature = "with-dioxus"))]
use crate::constants::KEY_CODE_ENTER;
#[cfg(not(feature = "with-dioxus"))]
use crate::presenters::MutPresenterActionValue;

use super::FormField;

#[cfg(not(feature = "with-dioxus"))]
use super::EventFn;

#[cfg(feature = "with-dioxus")]
#[component]
pub fn TextField<T: Clone + PartialEq + 'static>(
    #[props(optional)] error: Signal<Option<String>>,
    id: String,
    #[props(default = "text".to_owned())] input_type: String,
    #[props(optional)] mutation: Resource<MutPresenter<T>>,
    name: String,
    value: Signal<String>,
) -> Element {
    rsx! {
        FormField {
            error: error,
            for_id: &id,
            name: &name,
            mutation: mutation,
            input {
                class: if error().is_some() { "input w-full input-error" } else { "input w-full" },
                id: id,
                name: name,
                oninput: |event| {},
                r#type: input_type,
                value: value
            }
        }
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn TextField<D>(
    action_value: MutPresenterActionValue<D>,
    #[prop(optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(default = "text", into)] input_type: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
    #[prop(into, optional)] on_input: Option<EventFn>,
    #[prop(into, optional)] value: RwSignal<String>,
) -> impl IntoView
where
    D: Clone + Default + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();

    let _ = use_event_listener(node_ref, keydown, |event| {
        if event.key_code() == KEY_CODE_ENTER {
            event.prevent_default();
        }
    });

    let has_error = move || error.get().is_some();

    view! {
        <FormField action_value=action_value error=error id=id label=label name=name>
            <input
                class="input w-full"
                class:input-error=has_error
                id=id
                name=name
                node_ref=node_ref
                on:input=move |event| {
                    if let Some(on_input) = on_input.as_ref() {
                        on_input.0(event);
                    }
                }
                type=input_type
                bind:value=value
            />
        </FormField>
    }
}
