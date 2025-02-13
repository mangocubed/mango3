use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::icons::{EyeOutlined, EyeSlashOutlined};

#[component]
pub fn PasswordField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into)] label: Signal<&'static str>,
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
        <div class="form-control w-full">
            <label class="label" for=field_id>
                <span class="label-text">{move || label.get()}</span>
            </label>
            <div class="input input-bordered flex items-center gap-2 pr-0" class:input-error=has_error>
                <input class="grow" id=field_id name=name type=input_type />
                <button class="btn btn-ghost" type="button" on:click=toggle_type>
                    {move || {
                        if input_type.get() == "password" {
                            Either::Left(view! { <EyeSlashOutlined /> })
                        } else {
                            Either::Right(view! { <EyeOutlined /> })
                        }
                    }}
                </button>
            </div>
            <div class="label">
                <span class="label-text-alt text-error">{move || error.get()}</span>
            </div>
        </div>
    }
}
