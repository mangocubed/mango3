use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::LoadingSpinner;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_current_website_resource;

#[component]
pub fn CurrentWebsiteResource<VF, IV>(children: VF) -> impl IntoView
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
                    Some(Ok(website_opt)) => {
                        Either::Left(children_store.with_value(|store| store(website_opt)))
                    }
                    _ => Either::Right(()),
                }
            })}
        </Suspense>
    }
}
