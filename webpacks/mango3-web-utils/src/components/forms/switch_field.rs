use leptos::prelude::*;

use crate::presenters::{MutPresenter, MutPresenterActionValue};

use super::FieldError;

#[component]
pub fn SwitchField(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(optional, into)] is_checked: Signal<bool>,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
) -> impl IntoView {
    Effect::new(move || {
        let response = MutPresenter::from(action_value);

        if !name.is_empty() {
            error.set(response.error(name.to_owned()));
        }
    });

    let has_error = move || error.get().is_some();

    view! {
        <fieldset class="fieldset">
            <label class="fieldset-label" for=id>
                {label.run()}

                <input
                    class="toggle"
                    class:toggle-error=has_error
                    id=id
                    name=name
                    type="checkbox"
                    value="true"
                    checked=is_checked
                />
            </label>

            <FieldError error=error />
        </fieldset>
    }
}
