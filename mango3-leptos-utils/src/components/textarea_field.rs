use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_use::{use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn};

#[component]
pub fn TextareaField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into, optional)] label: TextProp,
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
        <div class="form-control w-full">
            <Show when={
                let label_is_present = !label.get().is_empty();
                move || label_is_present
            }>
                {
                    let label = label.clone();
                    move || {
                        let label = label.clone();
                        view! {
                            <label class="label" for=field_id>
                                <span class="label-text">{move || label.get()}</span>
                            </label>
                        }
                    }
                }
            </Show>

            <textarea
                node_ref=node_ref
                prop:value=content
                on:input=move |event| set_content.set(event_target_value(&event))
                class="textarea textarea-bordered font-mono"
                class:textarea-error=has_error
                id=field_id
                name=name
                rows=rows
            />
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
