use leptos::either::Either;
use leptos::ev::{keydown, MouseEvent};
use leptos::prelude::*;
use leptos_use::use_event_listener;

use crate::constants::KEY_CODE_ENTER;
use crate::icons::{EyeMini, EyeSlashMini};
use crate::presenters::MutPresenterActionValue;

use super::FormField;

#[component]
pub fn PasswordField<T>(
    action_value: MutPresenterActionValue<T>,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
) -> impl IntoView
where
    T: Clone + Default + Send + Sync + 'static,
{
    let node_ref = NodeRef::new();
    let input_type = RwSignal::new("password".to_owned());

    let _ = use_event_listener(node_ref, keydown, |event| {
        if event.key_code() == KEY_CODE_ENTER {
            event.prevent_default();
        }
    });

    let has_error = move || error.get().is_some();

    let toggle_type = move |event: MouseEvent| {
        event.prevent_default();

        input_type.update(|value| {
            *value = if value == "password" {
                "text".to_owned()
            } else {
                "password".to_owned()
            };
        });
    };

    view! {
        <FormField action_value=action_value error=error id=id label=label name=name>
            <div class="input flex items-center gap-2 pr-0 w-full" class:input-error=has_error>
                <input node_ref=node_ref class="grow" id=id name=name type=input_type />
                <button class="btn btn-ghost btn-sm" type="button" on:click=toggle_type>
                    {move || {
                        if input_type.get() == "password" {
                            Either::Left(view! { <EyeSlashMini /> })
                        } else {
                            Either::Right(view! { <EyeMini /> })
                        }
                    }}
                </button>
            </div>
        </FormField>
    }
}
