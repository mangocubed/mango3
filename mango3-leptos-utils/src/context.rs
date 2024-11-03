use leptos::prelude::*;
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};

use crate::models::BasicConfigResp;
use crate::server_functions::get_basic_config;

#[derive(Clone)]
pub struct PageTitle {
    pub value: RwSignal<Option<String>>,
}

pub fn provide_basic_config_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_basic_config()))
}

pub fn provide_page_title() {
    provide_context(PageTitle {
        value: RwSignal::new(None),
    })
}

pub fn use_basic_config_resource() -> Resource<Result<BasicConfigResp, ServerFnError>> {
    use_context::<Resource<Result<BasicConfigResp, ServerFnError>>>().unwrap()
}

pub fn use_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(
        UseColorModeOptions::default()
            .attribute("data-theme")
            .emit_auto(true)
            .cookie_enabled(true)
            .cookie_name("_mango3_color_mode")
            .transition_enabled(true),
    )
}

pub fn use_page_title() -> PageTitle {
    use_context::<PageTitle>().unwrap()
}
