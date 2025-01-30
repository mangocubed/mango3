use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::components::LoadingSpinner;
use mango3_leptos_utils::models::WebsiteResp;

use crate::context::use_my_website_resource;

mod highlight_code;
mod my_websites_infinite_scroll;
mod post_form_fields;
mod post_preview_modal;
mod selected_website_dropdown;
mod theme_selector_field;

pub use highlight_code::HighLightCode;
pub use my_websites_infinite_scroll::MyWebsitesInfiniteScroll;
pub use post_form_fields::PostFormFields;
pub use post_preview_modal::PostPreviewModal;
pub use selected_website_dropdown::SelectedWebsiteDropdown;
pub use theme_selector_field::ThemeSelectorField;

#[component]
pub fn MyWebsiteOpt<VF, IV>(children: VF) -> impl IntoView
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

#[component]
pub fn MyWebsite<VF, IV>(children: VF) -> impl IntoView
where
    IV: IntoView + 'static,
    VF: Fn(WebsiteResp) -> IV + Send + Sync + 'static,
{
    view! { <MyWebsiteOpt children=move |website_opt| { website_opt.map(&children) } /> }
}
