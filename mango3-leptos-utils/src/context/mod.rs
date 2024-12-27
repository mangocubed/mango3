use leptos::prelude::*;
use leptos_router::params::ParamsMap;

use crate::models::{BasicConfigResp, UserResp};
use crate::server_functions::get_current_user;

mod use_color_mode;
mod use_language_cookie;

pub use use_color_mode::{use_color_mode, use_color_mode_with_options, UseColorModeOptions};
pub use use_language_cookie::{use_language_cookie, use_language_cookie_options};

pub fn param_query(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get("q").unwrap_or_default())
}

pub fn provide_basic_config() {
    let basic_config = SharedValue::<BasicConfigResp>::new(move || {
        #[cfg(feature = "ssr")]
        {
            use mango3_core::config::BASIC_CONFIG;

            BASIC_CONFIG.clone().into()
        }

        #[cfg(not(feature = "ssr"))]
        {
            BasicConfigResp::default()
        }
    });

    provide_context(basic_config.into_inner());
}

pub fn provide_current_user_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_user()))
}

pub fn use_basic_config() -> BasicConfigResp {
    #[cfg(feature = "ssr")]
    {
        use mango3_core::config::BASIC_CONFIG;

        BASIC_CONFIG.clone().into()
    }

    #[cfg(not(feature = "ssr"))]
    {
        use_context::<BasicConfigResp>().unwrap()
    }
}

pub fn use_current_user_resource() -> Resource<Result<Option<UserResp>, ServerFnError>> {
    use_context::<Resource<Result<Option<UserResp>, ServerFnError>>>().unwrap()
}
