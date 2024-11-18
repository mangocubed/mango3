use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::LoadingSpinner;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_my_website_resource;

#[component]
pub fn MyWebsiteResource<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(Option<WebsiteResp>) -> IV + Send + Sync + 'static,
{
    let my_website_resource = use_my_website_resource();
    let children_store = StoredValue::new(children);

    view! {
        <Suspense fallback=LoadingSpinner>
            {move || Suspend::new(async move {
                match my_website_resource.get() {
                    Some(Ok(website_opt)) => Either::Left(children_store.with_value(|store| store(website_opt))),
                    _ => Either::Right(()),
                }
            })}
        </Suspense>
    }
}
