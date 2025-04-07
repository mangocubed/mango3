use leptos::ev::keydown;
use leptos::prelude::*;
use leptos_use::use_event_listener;

use crate::constants::KEY_CODE_ENTER;
use crate::presenters::MutPresenterActionValue;

use super::{EventFn, FormField};

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
