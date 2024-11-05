use leptos::either::EitherOf3;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn SubmitButton(
    #[prop(into)] is_loading: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let on_click = move |event: MouseEvent| {
        if is_loading.get() {
            event.prevent_default();
        }
    };

    view! {
        <div class="pt-2 pb-2">
            <button class="btn btn-block btn-primary" on:click=on_click type="submit">
                {move || {
                    if is_loading.get() {
                        EitherOf3::A(view! { <span class="loading loading-spinner" /> })
                    } else if let Some(children) = &children {
                        EitherOf3::B(children())
                    } else {
                        EitherOf3::C(move || tr!("submit"))
                    }
                }}
            </button>
        </div>
    }
}
