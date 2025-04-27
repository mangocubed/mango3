use dioxus::prelude::{use_context_provider, use_server_cached, use_server_future, Readable, RenderError};
use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

use crate::presenters::{AppConfigPresenter, InfoPresenter, RoutesPresenter};
use crate::server_functions::{get_app_config, get_basic_config, get_info};

pub fn provide_app_config_resource() -> Result<AppConfigPresenter, RenderError> {
    let resource = use_server_future(|| async { get_app_config().await.expect("Could not get App Config") })?;

    Ok(use_context_provider(|| resource).with(|config| config.clone().unwrap()))
}

pub fn provide_info() -> InfoPresenter {
    let resource = use_server_cached(|| { 
        #[cfg(feature = "server")]
        return mango3_core::utils::INFO.into();
        
        InfoPresenter::default()
    });

    use_context_provider(|| resource)
}

pub fn provide_i18n(language: LanguageIdentifier, extra_locales: Vec<(LanguageIdentifier, &'static str)>) -> I18n {
    use_init_i18n(|| {
        let mut i18n_config = I18nConfig::new(language)
            .with_locale((langid!("en"), include_str!("../../locales/en/shared.ftl")))
            .with_locale((langid!("es"), include_str!("../../locales/es/shared.ftl")))
            .with_locale((langid!("pt"), include_str!("../../locales/pt/shared.ftl")));

        for locale in extra_locales {
            i18n_config = i18n_config.with_locale(locale);
        }

        i18n_config
    })
}

pub fn provide_routes() -> RoutesPresenter {
    let resource = use_server_cached(|| {
        #[cfg(feature = "server")]
        return mango3_core::config::BASIC_CONFIG.into();
        
        RoutesPresenter::default()
    });
    
    use_context_provider(|| resource)
}