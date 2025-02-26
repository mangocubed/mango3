use leptos::prelude::*;

use mango3_leptos_utils::components::{
    InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController,
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
    let controller = InfiniteScrollLocalResourceController::new(|after| {
        LocalResource::new(move || async move { get_my_websites(after.get()).await })
    });
    let selected_website = use_selected_website();

    selected_website.set(None);

    view! {
        <InfiniteScroll controller=controller key=|website: &WebsitePreviewResp| website.id.clone() children=children />
    }
}
