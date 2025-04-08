use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use leptos_router::params::ParamsMap;
use mango3_web_utils::presenters::{WebsiteMinPresenter, WebsitePresenter};

use crate::constants::KEY_PARAM_WEBSITE_ID;
use crate::server_functions::get_my_website;

pub fn provide_selected_website() {
    provide_context::<ArcRwSignal<Option<WebsiteMinPresenter>>>(ArcRwSignal::new(None));
}

pub fn provide_my_website_resource() {
    let params_map = use_params_map();
    provide_context(LocalResource::new(move || async move {
        if let Some(id) = param_website_id(params_map) {
            get_my_website(id).await
        } else {
            Ok(None)
        }
    }))
}

pub fn use_selected_website() -> ArcRwSignal<Option<WebsiteMinPresenter>> {
    use_context::<ArcRwSignal<Option<WebsiteMinPresenter>>>().unwrap()
}

pub fn param_website_id(params_map: Memo<ParamsMap>) -> Option<String> {
    params_map.with(|params| params.get(KEY_PARAM_WEBSITE_ID))
}

pub fn use_my_website_resource() -> LocalResource<Result<Option<WebsitePresenter>, ServerFnError>> {
    use_context::<LocalResource<Result<Option<WebsitePresenter>, ServerFnError>>>().unwrap()
}
