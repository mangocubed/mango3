use leptos::prelude::*;
use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};

use crate::models::BasicConfigResp;

mod use_color_mode;

pub use use_color_mode::use_color_mode;

#[derive(Clone)]
pub struct PageTitle {
    pub value: RwSignal<Option<String>>,
}

pub fn provide_basic_config() {
    provide_context(SharedValue::<BasicConfigResp>::new(move || {
        #[cfg(feature = "ssr")]
        {
            use mango3_core::config::BASIC_CONFIG;

            BASIC_CONFIG.clone().into()
        }

        #[cfg(not(feature = "ssr"))]
        {
            BasicConfigResp::default()
        }
    }))
}

pub fn provide_page_title() {
    provide_context(PageTitle {
        value: RwSignal::new(None),
    })
}

pub fn use_basic_config() -> SharedValue<BasicConfigResp> {
    SharedValue::new(move || {
        #[cfg(feature = "ssr")]
        {
            use mango3_core::config::BASIC_CONFIG;

            BASIC_CONFIG.clone().into()
        }

        #[cfg(not(feature = "ssr"))]
        {
            BasicConfigResp::default()
        }
    })
}

pub fn use_color_mode_back() -> UseColorModeReturn {
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
