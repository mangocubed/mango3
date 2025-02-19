use leptos::prelude::*;
use leptos_use::{use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn};

#[component]
pub fn TextareaField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into, optional)] label: ViewFn,
    #[prop(default = 4, into)] rows: i8,
    #[prop(optional, into)] value: RwSignal<String>,
    name: &'static str,
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

    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

    let has_error = move || error.get().is_some();

    view! {
        <fieldset class="fieldset">
            <label class="fieldset-label empty:hidden" for=field_id>
                {label.run()}
            </label>

            <textarea
                node_ref=node_ref
                prop:value=content
                on:input=move |event| set_content.set(event_target_value(&event))
                class="textarea textarea-bordered font-mono w-full"
                class:textarea-error=has_error
                id=field_id
                name=name
                rows=rows
            />
            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
