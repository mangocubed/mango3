use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use leptos_router::params::ParamsMap;
use mango3_leptos_utils::models::WebsiteResp;

use crate::constants::KEY_PARAM_WEBSITE_ID;
use crate::server_functions::get_my_website;

pub fn provide_selected_website() {
    provide_context::<RwSignal<Option<WebsiteResp>>>(RwSignal::new(None));
}

pub fn provide_my_website_resource() {
    let params_map = use_params_map();
    provide_context(Resource::new_blocking(
        move || param_website_id(params_map),
        |website_id| async {
            if let Some(id) = website_id {
                get_my_website(id).await
            } else {
                Ok(None)
            }
        },
    ))
}

pub fn use_selected_website() -> RwSignal<Option<WebsiteResp>> {
    use_context::<RwSignal<Option<WebsiteResp>>>().unwrap()
}

pub fn param_website_id(params_map: Memo<ParamsMap>) -> Option<String> {
    params_map.with(|params| params.get(KEY_PARAM_WEBSITE_ID))
}

pub fn use_my_website_resource() -> Resource<Result<Option<WebsiteResp>, ServerFnError>> {
    use_context::<Resource<Result<Option<WebsiteResp>, ServerFnError>>>().unwrap()
}
