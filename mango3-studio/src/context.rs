use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::models::WebsiteResp;

use crate::constants::KEY_PARAM_WEBSITE_ID;
use crate::server_functions::get_my_website;

pub fn provide_my_website_resource() {
    let website_id = use_website_id_param();
    provide_context(Resource::new_blocking(move || website_id.clone(), get_my_website))
}

pub fn use_website_id_param() -> String {
    let params_map = use_params_map();

    params_map.with(|params| params.get(KEY_PARAM_WEBSITE_ID).unwrap())
}

pub fn use_my_website_resource() -> Resource<Result<Option<WebsiteResp>, ServerFnError>> {
    use_context::<Resource<Result<Option<WebsiteResp>, ServerFnError>>>().unwrap()
}
