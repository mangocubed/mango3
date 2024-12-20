use chrono::{DateTime, SecondsFormat, Utc};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::Meta;

use mango3_leptos_utils::components::LoadingSpinner;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_current_website_resource;

mod website_top_bar;

pub use website_top_bar::WebsiteTopBar;

#[component]
pub fn CurrentWebsiteOpt<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(Option<WebsiteResp>) -> IV + Send + Sync + 'static,
{
    let current_website_resource = use_current_website_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match current_website_resource.get() {
                    Some(Ok(website_opt)) => Either::Left(children_store.with_value(|store| store(website_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn CurrentWebsite<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsiteResp) -> IV + Send + Sync + 'static,
{
    view! { <CurrentWebsiteOpt children=move |website_opt| { website_opt.map(&children) } /> }
}

#[component]
pub fn HighLightCode(content: String) -> impl IntoView {
    let has_code = move || content.contains("</code>");

    view! {
        <Show when=has_code>
            <link
                rel="stylesheet"
                href="https://unpkg.com/@highlightjs/cdn-assets@11.11.0/styles/github-dark-dimmed.min.css"
            />
            <script type="module">
                r#"import hljs from "https://unpkg.com/@highlightjs/cdn-assets@11.11.0/es/highlight.min.js";
                document.body.querySelectorAll("pre code").forEach((element) => { hljs.highlightElement(element); });"#
            </script>
        </Show>
    }
}

#[component]
pub fn MetaDateTime(#[prop(into)] property: TextProp, content: DateTime<Utc>) -> impl IntoView {
    view! { <Meta property=property content=content.to_rfc3339_opts(SecondsFormat::Secs, true) /> }
}
