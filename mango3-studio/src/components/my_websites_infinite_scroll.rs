use leptos::prelude::*;

use mango3_leptos_utils::components::InfiniteScroll;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_selected_website;
use crate::server_functions::get_my_websites;

#[component]
pub fn MyWebsitesInfiniteScroll<IV, VF>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsiteResp) -> IV + Clone + Send + Sync + 'static,
{
    let after = RwSignal::new(None);
    let my_websites_resource = Resource::new_blocking(move || after.get(), get_my_websites);
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <InfiniteScroll
            after=after
            key=|website: &WebsiteResp| website.id.clone()
            resource=my_websites_resource
            children=children
        />
    }
}
