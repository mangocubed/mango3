use leptos::prelude::*;

#[component]
pub fn SwitchField<L>(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    label: L,
    name: &'static str,
    #[prop(optional, into)] is_checked: Signal<bool>,
) -> impl IntoView
where
    L: IntoView,
{
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
            <label class="fieldset-label" for=field_id>
                {label}

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

            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
