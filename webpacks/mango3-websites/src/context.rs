use leptos::prelude::*;
use leptos_router::params::ParamsMap;

use mango3_web_utils::presenters::{PostPresenter, WebsitePresenter};

use crate::constants::KEY_PARAM_SLUG;
use crate::server_functions::get_current_website;

pub fn param_slug(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get(KEY_PARAM_SLUG).unwrap_or_default())
}

pub fn provide_current_website_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_website()))
}

pub fn use_current_website_resource() -> Resource<Result<Option<WebsitePresenter>, ServerFnError>> {
    use_context::<Resource<Result<Option<WebsitePresenter>, ServerFnError>>>().unwrap()
}

pub fn use_current_post() -> PostPresenter {
    use_context::<PostPresenter>().unwrap()
}
