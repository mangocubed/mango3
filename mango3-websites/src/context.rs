use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::models::WebsiteResp;

use crate::constants::KEY_PARAM_SLUG;
use crate::server_functions::get_current_website;

pub fn provide_current_website_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_website()))
}

pub fn use_current_website_resource() -> Resource<Result<Option<WebsiteResp>, ServerFnError>> {
    use_context::<Resource<Result<Option<WebsiteResp>, ServerFnError>>>().unwrap()
}

pub fn use_slug_param() -> Memo<String> {
    let params_map = use_params_map();

    Memo::new(move |_| params_map.with(|params| params.get(KEY_PARAM_SLUG).unwrap_or_default()))
}
