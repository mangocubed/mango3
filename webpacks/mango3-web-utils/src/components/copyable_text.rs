use leptos::prelude::*;
use leptos_use::{use_clipboard, UseClipboardReturn};

use crate::i18n::use_i18n;
use crate::icons::LinkOutlined;

#[component]
pub fn CopyableText(#[prop(into)] value: String) -> impl IntoView {
    let UseClipboardReturn { copy, is_supported, .. } = use_clipboard();
    let i18n = use_i18n();

    view! {
        <div class="grow input flex items-center gap-2 pr-0 w-full">
            <input class="grow" value=value.clone() readonly />

            <Show when=move || is_supported.get()>
                <button
                    class="btn btn-ghost"
                    title=move || async_t_string!(i18n, shared.copy_url).get()
                    on:click={
                        let copy = copy.clone();
                        let value = value.clone();
                        move |event| {
                            event.prevent_default();
                            copy(&value);
                        }
                    }
                >
                    <LinkOutlined />
                </button>
            </Show>
        </div>
    }
}
