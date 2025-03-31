use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn Modal(
    is_open: RwSignal<bool>,
    children: Children,
    #[prop(into, optional)] box_class: &'static str,
    #[prop(into, optional)] class: &'static str,
    #[prop(into, optional)] on_close: Option<Callback<()>>,
    #[prop(default = true, into)] is_closable: bool,
) -> impl IntoView {
    let on_click_close = move |event: MouseEvent| {
        event.prevent_default();
        is_open.set(false);

        if let Some(oc) = on_close {
            oc.run(())
        }
    };

    view! {
        <dialog class=format!("modal {class}") class:modal-open=move || is_open.get()>
            <Show when=move || is_closable>
                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" on:click=on_click_close>
                    "✕"
                </button>
            </Show>

            <div class=format!("modal-box {box_class}")>{children()}</div>

            <Show when=move || is_closable>
                <div class="modal-backdrop" on:click=on_click_close />
            </Show>
        </dialog>
    }
}
