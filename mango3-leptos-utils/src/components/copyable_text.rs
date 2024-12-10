use leptos::prelude::*;
use leptos_use::{use_clipboard, UseClipboardReturn};

use crate::i18n::{t_string, use_i18n};
use crate::icons::LinkOutlined;

#[component]
pub fn CopyableText(#[prop(into)] value: String) -> impl IntoView {
    let UseClipboardReturn { copy, is_supported, .. } = use_clipboard();
    let i18n = use_i18n();

    view! {
        <div class="grow input input-bordered flex items-center gap-2 pr-0">
            <input class="grow" value=value.clone() readonly />

            <Show when=move || is_supported.get()>
                <button
                    class="btn btn-ghost"
                    title=t_string!(i18n, shared.copy_url)
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
