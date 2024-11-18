use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn TextareaField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: TextProp,
    #[prop(default = 4, into)] rows: i8,
    #[prop(optional, into)] value: Signal<String>,
    name: &'static str,
) -> impl IntoView {
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
            <label class="label" for=field_id>
                <span class="label-text">{move || label.get()}</span>
            </label>
            <textarea
                class="textarea textarea-bordered"
                class:textarea-error=has_error
                id=field_id
                name=name
                rows=rows
                prop:value=value
            />
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
