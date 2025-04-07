use leptos::prelude::*;
use leptos_router::params::ParamsMap;

use crate::constants::KEY_PARAM_NAME;
use crate::presenters::{BasicConfigPresenter, InfoPresenter};
use crate::server_functions::get_current_user;

#[cfg(feature = "current-user")]
use crate::presenters::UserPresenter;

mod use_color_mode;
mod use_language_cookie;

pub use use_color_mode::{use_color_mode, use_color_mode_with_options, UseColorModeOptions};
pub use use_language_cookie::{use_language_cookie, use_language_cookie_options};

pub fn param_name(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get(KEY_PARAM_NAME).unwrap_or_default())
}

pub fn param_query(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get("q").unwrap_or_default())
}

pub fn provide_basic_config() {
    let basic_config = SharedValue::<BasicConfigPresenter>::new(move || {
        #[cfg(feature = "ssr")]
        {
            use mango3_core::config::BASIC_CONFIG;

            BASIC_CONFIG.clone().into()
        }

        #[cfg(not(feature = "ssr"))]
        {
            BasicConfigPresenter::default()
        }
    });

    provide_context(basic_config.into_inner());
}

pub fn provide_info() {
    let info = SharedValue::<InfoPresenter>::new(move || {
        #[cfg(feature = "ssr")]
        {
            mango3_core::utils::INFO.clone().into()
        }

        #[cfg(not(feature = "ssr"))]
        {
            InfoPresenter::default()
        }
    });

    provide_context(info.into_inner());
}

pub fn provide_current_user_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_user()))
}

pub fn use_basic_config() -> BasicConfigPresenter {
    #[cfg(feature = "ssr")]
    {
        use mango3_core::config::BASIC_CONFIG;

        BASIC_CONFIG.clone().into()
    }

    #[cfg(not(feature = "ssr"))]
    {
        use_context::<BasicConfigPresenter>().unwrap()
    }
}

#[cfg(feature = "current-user")]
pub fn use_current_user_resource() -> Resource<Result<Option<UserPresenter>, ServerFnError>> {
    use_context::<Resource<Result<Option<UserPresenter>, ServerFnError>>>().unwrap()
}

pub fn use_info() -> InfoPresenter {
    #[cfg(feature = "ssr")]
    {
        mango3_core::utils::INFO.clone().into()
    }

    #[cfg(not(feature = "ssr"))]
    {
        use_context::<InfoPresenter>().unwrap()
    }
}
