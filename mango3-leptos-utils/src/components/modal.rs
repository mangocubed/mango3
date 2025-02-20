use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] is_open: RwSignal<bool>,
    children: Children,
    #[prop(into, optional)] box_class: &'static str,
    #[prop(into, optional)] class: &'static str,
    #[prop(default = true, into)] is_closable: bool,
) -> impl IntoView {
    view! {
        <dialog class=format!("modal {class}") class:modal-open=move || is_open.get()>
            <Show when=move || is_closable>
                <button
                    class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                    }
                >
                    "✕"
                </button>
            </Show>

            <div class=format!("modal-box {box_class}")>{children()}</div>

            <Show when=move || is_closable>
                <div class="modal-backdrop" on:click=move |_| is_open.set(false) />
            </Show>
        </dialog>
    }
}
