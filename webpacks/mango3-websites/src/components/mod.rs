use chrono::{DateTime, SecondsFormat, Utc};
use leptos::either::Either;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_meta::Meta;

use mango3_web_utils::models::WebsiteResp;

use crate::context::use_current_website_resource;

mod highlight_code;
mod post_comments;
mod post_reactions;
mod website_top_bar;

pub use highlight_code::HighLightCode;
pub use post_comments::PostComments;
pub use post_reactions::PostReactions;
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
        <Transition>
            {move || Suspend::new(async move {
                match current_website_resource.get() {
                    Some(Ok(website_opt)) => Either::Left(children_store.with_value(|store| store(website_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Transition>
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
pub fn MetaDateTime(#[prop(into)] property: TextProp, content: DateTime<Utc>) -> impl IntoView {
    view! { <Meta property=property content=content.to_rfc3339_opts(SecondsFormat::Secs, true) /> }
}
