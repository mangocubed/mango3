use leptos::ev::{keydown, keyup, MouseEvent};
use leptos::prelude::*;
use leptos_use::{
    use_event_listener, use_textarea_autosize_with_options, UseTextareaAutosizeOptions, UseTextareaAutosizeReturn,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDocument, HtmlTextAreaElement};

use crate::constants::{KEY_CODE_5, KEY_CODE_B, KEY_CODE_I, KEY_CODE_K};
use crate::i18n::use_i18n;
use crate::icons::{
    ArrowUturnLeftMini, ArrowUturnRightMini, BoldMini, ImageMini, ItalicMini, LinkMini, StrikethroughMini,
};
use crate::presenters::MutPresenterActionValue;

use super::FormField;

mod image_modal;
mod link_modal;

use image_modal::ImageModal;
use link_modal::LinkModal;

const BOLD: &str = "**";
const ITALIC: &str = "_";
const STRIKETHROUGH: &str = "~";

#[component]
pub fn MarkdownEditorField(
    #[prop(optional)] action_value: MutPresenterActionValue,
    #[prop(into, optional)] error: RwSignal<Option<String>>,
    #[prop(into, optional)] id: &'static str,
    #[prop(into, optional)] label: ViewFn,
    #[prop(into, optional)] name: &'static str,
    #[prop(default = 4, into)] rows: i8,
    #[prop(optional, into)] value: RwSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let node_ref = NodeRef::new();
    let hotkey_pressed = RwSignal::new(false);
    let image_modal_is_open = RwSignal::new(false);
    let link_modal_is_open = RwSignal::new(false);

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

    let text_selection = move || {
        let el = textarea_element();
        let sel_start = el.selection_start().ok().flatten().unwrap_or_default() as usize;
        let sel_end = el.selection_end().ok().flatten().unwrap_or_default() as usize;

        if sel_end >= sel_start {
            (sel_start, sel_end)
        } else {
            (sel_end, sel_start)
        }
    };

    let selected_text = move || {
        let (sel_start, sel_end) = text_selection();

        value.get().get(sel_start..sel_end).unwrap_or_default().to_owned()
    };

    let insert_text = move |text: &str, cursor_pos: usize| {
        let el = textarea_element();
        let (sel_start, _) = text_selection();
        let new_position = (sel_start + cursor_pos) as u32;

        let _ = el.focus();
        let _ = html_document().exec_command_with_show_ui_and_value("insertText", false, text);
        let _ = el.set_selection_range(new_position, new_position);

        set_content.set(el.value());
    };

    let insert_wrap = move |chars: &str| {
        let text = selected_text();
        insert_text(&format!("{chars}{text}{chars}",), chars.len() + text.len());
    };

    let _ = use_event_listener(node_ref, keydown, move |event| {
        if hotkey_pressed.get() {
            event.prevent_default();
            return;
        }

        if event.ctrl_key() {
            match event.key_code() {
                KEY_CODE_B => {
                    event.prevent_default();
                    insert_wrap(BOLD);
                }
                KEY_CODE_I => {
                    event.prevent_default();
                    insert_wrap(ITALIC);
                }
                KEY_CODE_K => {
                    event.prevent_default();
                    link_modal_is_open.set(true);
                }
                _ => {
                    hotkey_pressed.set(false);
                    return;
                }
            }
        } else if event.alt_key() && event.shift_key() {
            match event.key_code() {
                KEY_CODE_5 => {
                    event.prevent_default();
                    insert_wrap(STRIKETHROUGH);
                }
                KEY_CODE_I => {
                    event.prevent_default();
                    image_modal_is_open.set(true);
                }
                _ => {
                    hotkey_pressed.set(false);
                    return;
                }
            }
        } else {
            hotkey_pressed.set(false);
            return;
        }

        hotkey_pressed.set(true);
    });

    let _ = use_event_listener(node_ref, keyup, move |event| {
        event.prevent_default();
        hotkey_pressed.set(false);
    });

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
        insert_wrap(BOLD);
    };

    let on_click_italic = move |event: MouseEvent| {
        event.prevent_default();
        insert_wrap(ITALIC);
    };

    let on_click_strikethrough = move |event: MouseEvent| {
        event.prevent_default();
        insert_wrap(STRIKETHROUGH);
    };

    let on_click_link = move |event: MouseEvent| {
        event.prevent_default();
        link_modal_is_open.set(true);
    };

    let on_click_image = move |event: MouseEvent| {
        event.prevent_default();
        image_modal_is_open.set(true);
    };

    view! {
        <FormField action_value=action_value error=error id=id label=label name=name>
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
                            "{} (Alt + Shift + S)",
                            async_t_string!(i18n, shared.strikethrough).with(|value| value.unwrap_or("Strikethrough")),
                        )
                    }
                >
                    <StrikethroughMini />
                </button>

                <div class="divider divider-horizontal mx-1" />

                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_link
                    title=move || {
                        format!(
                            "{} (Ctrl + K)",
                            async_t_string!(i18n, shared.insert_link).with(|value| value.unwrap_or("Insert link")),
                        )
                    }
                >
                    <LinkMini />
                </button>
                <button
                    class="btn btn-sm btn-outline btn-accent px-2"
                    on:click=on_click_image
                    title=move || {
                        format!(
                            "{} (Alt + Shift + I)",
                            async_t_string!(i18n, shared.insert_image).with(|value| value.unwrap_or("Insert image")),
                        )
                    }
                >
                    <ImageMini />
                </button>
            </div>

            <LinkModal insert_text=insert_text is_open=link_modal_is_open selected_text=selected_text />

            <ImageModal insert_text=insert_text is_open=image_modal_is_open selected_text=selected_text />

            <textarea
                node_ref=node_ref
                prop:value=content
                on:input=move |event| set_content.set(event_target_value(&event))
                class="textarea textarea-bordered font-mono w-full"
                class:textarea-error=has_error
                id=id
                name=name
                rows=rows
            />
        </FormField>
    }
}
