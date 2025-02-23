use leptos::prelude::*;

use crate::components::Modal;

#[component]
pub fn ActionFormErrorAlert(#[prop(into)] is_active: Signal<bool>, #[prop(into)] message: ViewFn) -> impl IntoView {
    view! {
        <Show when=move || is_active.get()>
            <div class="py-2 has-[div:empty]:hidden">
                <div role="alert" class="alert alert-error">
                    {message.run()}
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn ActionFormSuccessModal(
    is_open: RwSignal<bool>,
    #[prop(into)] message: ViewFn,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <Modal is_closable=false is_open=is_open on_close=on_close>
            <div>{message.run()}</div>
            <div class="modal-action">
                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                        on_close.run(());
                    }
                >
                    "Ok"
                </button>
            </div>
        </Modal>
    }
}
