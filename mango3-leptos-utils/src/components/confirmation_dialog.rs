use leptos::prelude::*;

use crate::i18n::{t, use_i18n};

#[component]
pub fn ConfirmationDialog<OA>(children: Children, #[prop(into)] is_open: RwSignal<bool>, on_accept: OA) -> impl IntoView
where
    OA: Fn() + Send + Sync + 'static,
{
    let i18n = use_i18n();

    view! {
        <div class="modal" class:modal-open=is_open>
            <div class="modal-box">
                <div>{children()}</div>

                <div class="modal-action">
                    <button class="btn" on:click=move |_| is_open.set(false)>
                        {t!(i18n, shared.cancel)}
                    </button>
                    <button
                        class="btn btn-primary"
                        on:click=move |_| {
                            is_open.set(false);
                            on_accept()
                        }
                    >
                        {t!(i18n, shared.accept)}
                    </button>
                </div>
            </div>
        </div>
    }
}
