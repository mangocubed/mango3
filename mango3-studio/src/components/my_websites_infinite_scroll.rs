use leptos::prelude::*;

use mango3_leptos_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollResourceController,
};
use mango3_leptos_utils::models::WebsitePreviewResp;

use crate::context::use_selected_website;
use crate::server_functions::get_my_websites;

#[component]
pub fn MyWebsitesInfiniteScroll<IV, VF>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsitePreviewResp) -> IV + Clone + Send + Sync + 'static,
{
    let controller =
        InfiniteScrollResourceController::new(|after| Resource::new_blocking(move || after.get(), get_my_websites));
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <InfiniteScroll controller=controller key=|website: &WebsitePreviewResp| website.id.clone() children=children />
    }
}
