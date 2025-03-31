use leptos::prelude::*;

use crate::components::Modal;

#[component]
pub fn AlertModal<OC>(children: Children, #[prop(into)] is_open: RwSignal<bool>, on_close: OC) -> impl IntoView
where
    OC: Fn() + Send + Sync + 'static,
{
    let is_done = RwSignal::new(false);

    Effect::new(move || {
        if is_open.get() {
            is_done.set(true);
        }
    });

    Effect::new(move || {
        if !is_open.get() && is_done.get() {
            on_close()
        }
    });

    view! {
        <Modal is_closable=false is_open=is_open>
            <div>{children()}</div>

            <div class="modal-action">
                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                    }
                >
                    "Ok"
                </button>
            </div>
        </Modal>
    }
}
