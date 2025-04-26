#[cfg(feature = "with-dioxus")]
use dioxus::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;

#[cfg(not(feature = "with-dioxus"))]
mod arrow;
#[cfg(not(feature = "with-dioxus"))]
mod bars;
#[cfg(not(feature = "with-dioxus"))]
mod chevron;
#[cfg(not(feature = "with-dioxus"))]
mod document;
#[cfg(not(feature = "with-dioxus"))]
mod exclamation;
#[cfg(not(feature = "with-dioxus"))]
mod eye;
#[cfg(not(feature = "with-dioxus"))]
mod home;
#[cfg(not(feature = "with-dioxus"))]
mod image;
#[cfg(not(feature = "with-dioxus"))]
mod information_circle;
#[cfg(not(feature = "with-dioxus"))]
mod link;
#[cfg(not(feature = "with-dioxus"))]
mod magnifying_glass;
#[cfg(not(feature = "with-dioxus"))]
mod paper_clip;
#[cfg(not(feature = "with-dioxus"))]
mod pencil;
#[cfg(not(feature = "with-dioxus"))]
mod plus;
#[cfg(not(feature = "with-dioxus"))]
mod text_editor;
#[cfg(not(feature = "with-dioxus"))]
mod trash;
#[cfg(not(feature = "with-dioxus"))]
mod users;

#[cfg(not(feature = "with-dioxus"))]
pub use arrow::{ArrowUturnLeftMini, ArrowUturnRightMini};
#[cfg(not(feature = "with-dioxus"))]
pub use bars::Bars3Outlined;
#[cfg(not(feature = "with-dioxus"))]
pub use chevron::{ChevronDownMini, ChevronUpMini};
#[cfg(not(feature = "with-dioxus"))]
pub use document::{DocumentOutlined, DocumentTextOutlined};
#[cfg(not(feature = "with-dioxus"))]
pub use exclamation::ExclamationOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use eye::{EyeMini, EyeSlashMini};
#[cfg(not(feature = "with-dioxus"))]
pub use home::HomeOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use image::ImageMini;
#[cfg(not(feature = "with-dioxus"))]
pub use information_circle::InformationCircleOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use link::{LinkMini, LinkOutlined};
#[cfg(not(feature = "with-dioxus"))]
pub use magnifying_glass::MagnifyingGlassMini;
#[cfg(not(feature = "with-dioxus"))]
pub use paper_clip::PaperClipOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use pencil::PencilSquareOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use plus::PlusOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use text_editor::{BoldMini, ItalicMini, StrikethroughMini};
#[cfg(not(feature = "with-dioxus"))]
pub use trash::TrashOutlined;
#[cfg(not(feature = "with-dioxus"))]
pub use users::UsersOutlined;

#[cfg(feature = "with-dioxus")]
#[component]
pub fn Bars3Outlined() -> Element {
    rsx! {
        svg { fill: "none", view_box: "0 0 24 24", "stroke-width": "1.5", stroke: "currentColor", class: "size-6",
          path { "stroke-linecap": "round", "stroke-linejoin": "round", d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" }
        }
    }
}

#[cfg(feature = "with-dioxus")]
#[component]
pub fn ChevronUpMini() -> Element {
    rsx! {
        svg {
            class: "size-5",
            fill: "currentColor",
            view_box: "0 0 20 20",
            path {
                clip_rule: "evenodd",
                d: "M9.47 6.47a.75.75 0 0 1 1.06 0l4.25 4.25a.75.75 0 1 1-1.06 1.06L10 8.06l-3.72 3.72a.75.75 0 0 1-1.06-1.06l4.25-4.25Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn CheckMini() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
            <path
                fill-rule="evenodd"
                d="M16.704 4.153a.75.75 0 0 1 .143 1.052l-8 10.5a.75.75 0 0 1-1.127.075l-4.5-4.5a.75.75 0 0 1 1.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 0 1 1.05-.143Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn ComputerOutlined() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25"
            />
        </svg>
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn MoonOutlined() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"
            />
        </svg>
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn SunOutlined() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
            />
        </svg>
    }
}
