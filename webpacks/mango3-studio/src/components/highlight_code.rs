use leptos::prelude::*;
use leptos_use::{ColorMode, UseColorModeReturn};

use mango3_web_utils::context::use_color_mode;

#[component]
pub fn HighLightCode(#[prop(into)] content: String) -> impl IntoView {
    let has_code = move || content.contains("</code>");

    view! {
        <Show when=has_code>
            {move || {
                let UseColorModeReturn { state, .. } = use_color_mode();
                let style_link = move || {
                    if state.get() == ColorMode::Dark {
                        "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/github-dark.min.css"
                    } else {
                        "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/styles/github.min.css"
                    }
                };
                view! {
                    <link rel="stylesheet" href=style_link />
                    <script type="module">
                        r#"import hljs from "https://unpkg.com/@highlightjs/cdn-assets@11.11.1/es/highlight.min.js";
                        document.body.querySelectorAll("pre code").forEach((element) => { hljs.highlightElement(element); });"#
                    </script>
                }
            }}
        </Show>
    }
}
