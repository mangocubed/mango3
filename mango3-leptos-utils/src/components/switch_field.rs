use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn SwitchField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: TextProp,
    name: &'static str,
    #[prop(optional, into)] is_checked: Signal<bool>,
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
                <input
                    class="toggle"
                    class:toggle-error=has_error
                    id=field_id
                    name=name
                    type="checkbox"
                    value="true"
                    checked=is_checked
                />
            </label>
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
