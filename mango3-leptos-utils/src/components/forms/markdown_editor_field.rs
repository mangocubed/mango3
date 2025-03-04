use leptos::ev::{keydown, keyup, MouseEvent};
use leptos::prelude::*;
use leptos_use::{
    use_event_listener, use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

use crate::i18n::use_i18n;
use crate::icons::{ArrowUturnLeftMini, ArrowUturnRightMini, BoldMini, ItalicMini, StrikethroughMini};

const BOLD: &str = "**";
const ITALIC: &str = "_";
const STRIKETHROUGH: &str = "~";

#[component]
pub fn MarkdownEditorField(
    #[prop(into, optional)] error: MaybeProp<String>,
    #[prop(into, optional)] id: Option<&'static str>,
    #[prop(into, optional)] label: ViewFn,
    #[prop(default = 4, into)] rows: i8,
    #[prop(optional, into)] value: RwSignal<String>,
    name: &'static str,
) -> impl IntoView {
    let i18n = use_i18n();
    let node_ref = NodeRef::new();
    let hotkey_pressed = RwSignal::new(false);

    let UseTextareaAutosizeReturn {
        content, set_content, ..
    } = use_textarea_autosize_with_options(
        node_ref,
        UseTextareaAutosizeOptions::default()
            .style_prop("min-height")
            .content(value),
    );

    let html_document = move || {
        document()
            .dyn_into::<HtmlDocument>()
            .expect("Could not cast to HtmlDocument")
    };
    let textarea_element = move || node_ref.get().expect("Could not get element") as HtmlTextAreaElement;

    let set_wrap = move |chars: &str| {
        let el = textarea_element();
        let mut sel_start = el.selection_start().ok().flatten().unwrap_or_default() as usize;
        let mut sel_end = el.selection_end().ok().flatten().unwrap_or_default() as usize;
        let chars_len = chars.len();

        if sel_start > sel_end {
            (sel_start, sel_end) = (sel_end, sel_start);
        }

        let _ = el.focus();
        let _ = html_document().exec_command_with_show_ui_and_value(
            "insertText",
            false,
            &format!("{chars}{}{chars}", &value.get()[sel_start..sel_end]),
        );
        let _ = el.set_selection_range((sel_start + chars_len) as u32, (sel_end + chars_len) as u32);

        set_content.set(el.value());
    };

    let _ = use_event_listener(node_ref, keydown, move |event| {
        if hotkey_pressed.get() {
            event.prevent_default();
            return;
        }

        if event.ctrl_key() && event.key().to_lowercase() == "b" {
            event.prevent_default();
            set_wrap(BOLD);
        } else if event.ctrl_key() && event.key().to_lowercase() == "i" {
            event.prevent_default();
            set_wrap(ITALIC);
        } else if event.ctrl_key() && event.alt_key() && event.key().to_lowercase() == "s" {
            event.prevent_default();
            set_wrap(STRIKETHROUGH);
        } else {
            return;
        }

        hotkey_pressed.set(true);
    });

    let _ = use_event_listener(node_ref, keyup, move |event| {
        event.prevent_default();
        hotkey_pressed.set(false);
    });

    let field_id = move || {
        if let Some(id) = id {
            id.to_owned()
        } else {
            format!("field-{name}")
        }
    };

    let has_error = move || error.get().is_some();

    let on_click_undo = move |event: MouseEvent| {
        event.prevent_default();

        let _ = textarea_element().focus();
        let _ = html_document().exec_command("undo");
    };

    let on_click_redo = move |event: MouseEvent| {
        event.prevent_default();

        let _ = textarea_element().focus();
        let _ = html_document().exec_command("redo");
    };

    let on_click_bold = move |event: MouseEvent| {
        event.prevent_default();
        set_wrap(BOLD);
    };

    let on_click_italic = move |event: MouseEvent| {
        event.prevent_default();
        set_wrap(ITALIC);
    };

    let on_click_strikethrough = move |event: MouseEvent| {
        event.prevent_default();
        set_wrap(STRIKETHROUGH);
    };

    view! {
        <fieldset class="fieldset">
            <label class="fieldset-label empty:hidden" for=field_id>
                {label.run()}
            </label>

            <div class="join">
                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_undo
                    title=move || {
                        format!(
                            "{} (Ctrl + Z)",
                            async_t_string!(i18n, shared.undo).with(|value| value.unwrap_or("Undo")),
                        )
                    }
                >
                    <ArrowUturnLeftMini />
                </button>
                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_redo
                    title=move || {
                        format!(
                            "{} (Ctrl + Shift + Z)",
                            async_t_string!(i18n, shared.redo).with(|value| value.unwrap_or("Redo")),
                        )
                    }
                >
                    <ArrowUturnRightMini />
                </button>

                <div class="divider divider-horizontal mx-1" />

                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_bold
                    title=move || {
                        format!(
                            "{} (Ctrl + B)",
                            async_t_string!(i18n, shared.bold).with(|value| value.unwrap_or("Bold")),
                        )
                    }
                >
                    <BoldMini />
                </button>
                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_italic
                    title=move || {
                        format!(
                            "{} (Ctrl + I)",
                            async_t_string!(i18n, shared.italic).with(|value| value.unwrap_or("Italic")),
                        )
                    }
                >
                    <ItalicMini />
                </button>
                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_strikethrough
                    title=move || {
                        format!(
                            "{} (Ctrl + Alt + S)",
                            async_t_string!(i18n, shared.strikethrough).with(|value| value.unwrap_or("Strikethrough")),
                        )
                    }
                >
                    <StrikethroughMini />
                </button>
            </div>

            <textarea
                node_ref=node_ref
                prop:value=content
                on:input=move |event| set_content.set(event_target_value(&event))
                class="textarea textarea-bordered font-mono w-full"
                class:textarea-error=has_error
                id=field_id
                name=name
                rows=rows
            />
            <div class="fieldset-label text-error">{move || error.get()}</div>
        </fieldset>
    }
}
