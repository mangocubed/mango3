#[cfg(feature = "with-dioxus")]
use dioxus::prelude::{use_context, use_context_provider, use_server_future, Readable, RenderError, Resource};
#[cfg(feature = "with-dioxus")]
use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos_router::params::ParamsMap;
#[cfg(feature = "with-dioxus")]
use unic_langid::{LanguageIdentifier, langid};

use crate::presenters::{BasicConfigPresenter, InfoPresenter};
use crate::server_functions::get_basic_config;

#[cfg(not(feature = "with-dioxus"))]
use crate::constants::KEY_PARAM_NAME;
#[cfg(not(feature = "with-dioxus"))]
use crate::server_functions::get_current_user;

#[cfg(all(not(feature = "with-dioxus"), feature = "current-user"))]
use crate::presenters::UserPresenter;

#[cfg(not(feature = "with-dioxus"))]
mod use_color_mode;
#[cfg(not(feature = "with-dioxus"))]
mod use_language_cookie;

#[cfg(not(feature = "with-dioxus"))]
pub use use_color_mode::{use_color_mode, use_color_mode_with_options, UseColorModeOptions};
#[cfg(not(feature = "with-dioxus"))]
pub use use_language_cookie::{use_language_cookie, use_language_cookie_options};

#[cfg(not(feature = "with-dioxus"))]
pub fn param_name(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get(KEY_PARAM_NAME).unwrap_or_default())
}

#[cfg(not(feature = "with-dioxus"))]
pub fn param_query(params_map: Memo<ParamsMap>) -> String {
    params_map.with(|params| params.get("q").unwrap_or_default())
}

#[cfg(feature = "with-dioxus")]
pub fn provide_basic_config() -> Result<BasicConfigPresenter, RenderError> {
    let resource = use_server_future(|| async { get_basic_config().await.expect("Could not get basic config") })?;

    Ok(use_context_provider(|| resource).with(|config| config.clone().unwrap()))
}

#[cfg(not(feature = "with-dioxus"))]
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

    use_context_provider(basic_config.into_inner());
}

#[cfg(feature = "with-dioxus")]
pub fn provide_i18n(extra_locales: Vec<(LanguageIdentifier, &str)>) -> I18n {
    use_init_i18n(|| {
        let mut i18n_config = I18nConfig::new(langid!("en"))
            .with_locale((langid!("en"), include_str!("../../../locales/en/shared.ftl")))
            .with_locale((langid!("es"), include_str!("../../../locales/es/shared.ftl")))
            .with_locale((langid!("pt"), include_str!("../../../locales/pt/shared.ftl")));
            
        for locale in extra_locales {
            i18n_config = i18n_config.with_locale(locale);
        }
            
        i18n_config
    })
}

#[cfg(not(feature = "with-dioxus"))]
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

#[cfg(not(feature = "with-dioxus"))]
pub fn provide_current_user_resource() {
    provide_context(Resource::new_blocking(|| (), |_| get_current_user()))
}

#[cfg(feature = "with-dioxus")]
pub fn use_basic_config() -> BasicConfigPresenter {
    use_context::<Resource<BasicConfigPresenter>>().with(|config| config.clone().unwrap())
}

#[cfg(not(feature = "with-dioxus"))]
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

#[cfg(all(not(feature = "with-dioxus"), feature = "current-user"))]
pub fn use_current_user_resource() -> Resource<Result<Option<UserPresenter>, ServerFnError>> {
    use_context::<Resource<Result<Option<UserPresenter>, ServerFnError>>>().unwrap()
}

#[cfg(not(feature = "with-dioxus"))]
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
