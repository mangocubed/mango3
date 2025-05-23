use leptos::ev::MouseEvent;
use leptos::prelude::*;

use crate::components::forms::TextField;
use crate::components::Modal;
use crate::i18n::{t, use_i18n};
use crate::presenters::MutPresenterActionValue;

#[component]
pub fn ImageModal<IT, ST>(insert_text: IT, is_open: RwSignal<bool>, selected_text: ST) -> impl IntoView
where
    IT: Fn(&str, usize) + Send + 'static,
    ST: Fn() -> String + 'static,
{
    let i18n = use_i18n();
    let dummy_action_value = MutPresenterActionValue::<()>::new(None);
    let value_url = RwSignal::new("".to_string());
    let value_alt_text = RwSignal::new("".to_string());

    Effect::new(move |_| {
        if is_open.get() {
            value_url.set(String::new());
            value_alt_text.set(selected_text());
        }
    });

    let on_click_accept = move |event: MouseEvent| {
        event.prevent_default();
        let text = format!("![{}]({})", value_alt_text.get(), value_url.get());
        insert_text(&text, text.len());
        is_open.set(false);
    };

    let on_click_cancel = move |event: MouseEvent| {
        event.prevent_default();
        is_open.set(false);
    };

    let on_close = move || {
        value_url.set(String::new());
        value_alt_text.set(String::new());
    };

    view! {
        <Modal is_open=is_open on_close=on_close>
            <h4 class="h4">{t!(i18n, shared.insert_image)}</h4>

            <TextField action_value=dummy_action_value label=move || t!(i18n, shared.url) value=value_url />
            <TextField
                action_value=dummy_action_value
                label=move || t!(i18n, shared.alternative_text)
                value=value_alt_text
            />

            <div class="modal-action">
                <button class="btn" on:click=on_click_cancel>
                    {t!(i18n, shared.cancel)}
                </button>
                <button class="btn btn-primary" on:click=on_click_accept>
                    {t!(i18n, shared.accept)}
                </button>
            </div>
        </Modal>
    }
}
