use leptos::prelude::*;

use crate::i18n::{t, use_i18n};

use super::Modal;

#[component]
pub fn ConfirmationModal<OA>(children: Children, #[prop(into)] is_open: RwSignal<bool>, on_accept: OA) -> impl IntoView
where
    OA: Fn() + Send + Sync + 'static,
{
    let i18n = use_i18n();

    view! {
        <Modal is_closable=false is_open=is_open>
            <div>{children()}</div>

            <div class="modal-action">
                <button
                    class="btn"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                    }
                >
                    {t!(i18n, shared.cancel)}
                </button>
                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        is_open.set(false);
                        on_accept()
                    }
                >
                    {t!(i18n, shared.accept)}
                </button>
            </div>
        </Modal>
    }
}
