use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::icons::{EyeMini, EyeSlashMini};

#[component]
pub fn PasswordField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: ViewFn,
    name: &'static str,
) -> impl IntoView {
    let input_type = RwSignal::new("password".to_owned());
    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

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
        <fieldset class="fieldset">
            <label class="fieldset-label" for=field_id>
                {label.run()}
            </label>

            <div class="input flex items-center gap-2 pr-0 w-full" class:input-error=has_error>
                <input class="grow" id=field_id name=name type=input_type />
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

            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
