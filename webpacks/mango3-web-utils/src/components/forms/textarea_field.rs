use leptos::prelude::*;
use leptos_use::{use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn};

use crate::presenters::MutPresenterActionValue;

use super::FormField;

#[component]
pub fn TextareaField(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
    #[prop(default = 4, into)] rows: i8,
    #[prop(optional, into)] value: RwSignal<String>,
) -> impl IntoView {
    let node_ref = NodeRef::new();

    let UseTextareaAutosizeReturn {
        content, set_content, ..
    } = use_textarea_autosize_with_options(
        node_ref,
        UseTextareaAutosizeOptions::default()
            .style_prop("min-height")
            .content(value),
    );

    let has_error = move || error.get().is_some();

    view! {
        <FormField action_value=action_value error=error id=id label=label name=name>
            <textarea
                node_ref=node_ref
                prop:value=content
                on:input=move |event| set_content.set(event_target_value(&event))
                class="textarea textarea-bordered font-mono w-full"
                class:textarea-error=has_error
                id=id
                name=name
                rows=rows
            />
        </FormField>
    }
}
