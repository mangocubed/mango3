use leptos::{ev::keydown, prelude::*};
use leptos_use::use_event_listener;

use super::EventFn;

#[component]
pub fn TextField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(default = "text", into)] input_type: &'static str,
    #[prop(into)] label: ViewFn,
    name: &'static str,
    #[prop(into, optional)] on_input: Option<EventFn>,
    #[prop(optional, into)] value: RwSignal<String>,
) -> impl IntoView {
    let node_ref = NodeRef::new();

    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

    let _ = use_event_listener(node_ref, keydown, |event| {
        if event.key_code() == 13 {
            event.prevent_default();
        }
    });

    let has_error = move || error.get().is_some();

    view! {
        <fieldset class="fieldset">
            <label class="fieldset-label" for=field_id>
                {label.run()}
            </label>

            <input
                class="input w-full"
                class:input-error=has_error
                id=field_id
                name=name
                node_ref=node_ref
                on:input=move |event| {
                    if let Some(on_input) = on_input.as_ref() {
                        on_input.0(event)
                    }
                }
                type=input_type
                bind:value=value
            />

            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
