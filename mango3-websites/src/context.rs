use leptos::prelude::*;

use mango3_leptos_utils::models::WebsiteResp;

use crate::server_functions::get_current_website;

pub fn provide_current_website_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_website()))
}

pub fn use_current_website_resource() -> Resource<Result<Option<WebsiteResp>, ServerFnError>> {
    use_context::<Resource<Result<Option<WebsiteResp>, ServerFnError>>>().unwrap()
}
