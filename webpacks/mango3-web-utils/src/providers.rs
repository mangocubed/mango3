use dioxus::prelude::{use_context_provider, use_resource, use_server_cached, RenderError, Resource};
use dioxus_i18n::prelude::{use_init_i18n, I18n, I18nConfig};
use unic_langid::{langid, LanguageIdentifier};

use crate::presenters::{AppConfigPresenter, AppInfoPresenter, AppRoutesPresenter};
use crate::server_functions::get_app_config;

pub fn provide_app_config_resource() -> Result<Resource<AppConfigPresenter>, RenderError> {
    let resource = use_resource(|| async { get_app_config().await.expect("Could not get app config") });

    Ok(use_context_provider(|| resource))
}

pub fn provide_app_info() -> AppInfoPresenter {
    let resource = use_server_cached(|| {
        #[cfg(feature = "server")]
        {
            let basic_config = &mango3_core::config::BASIC_CONFIG;
            let info = &mango3_core::utils::INFO;

            return AppInfoPresenter {
                copyright: basic_config.copyright.clone(),
                description: basic_config.description.clone(),
                domain: basic_config.domain.clone(),
                enable_register: basic_config.enable_register,
                git_commit_hash: info.git_commit_hash.clone(),
                git_commit_short_hash: info.git_commit_short_hash.clone(),
                reaction_emojis: info.reaction_emojis.clone(),
                support_email_address: basic_config.support_email_address.clone(),
                title: basic_config.title.clone(),
                version: info.version.clone(),
            };
        }

        AppInfoPresenter::default()
    });

    use_context_provider(|| resource)
}

pub fn provide_i18n(extra_locales: Vec<(LanguageIdentifier, &'static str)>) -> I18n {
    use_init_i18n(|| {
        let mut i18n_config = I18nConfig::new(langid!("en"))
            .with_fallback(langid!("en"))
            .with_locale((langid!("en"), include_str!("../../locales/en/shared.ftl")))
            .with_locale((langid!("es"), include_str!("../../locales/es/shared.ftl")))
            .with_locale((langid!("pt"), include_str!("../../locales/pt/shared.ftl")));

        for locale in extra_locales {
            i18n_config = i18n_config.with_locale(locale);
        }

        i18n_config
    })
}

pub fn provide_app_routes() -> AppRoutesPresenter {
    let resource = use_server_cached(|| {
        #[cfg(feature = "server")]
        return mango3_core::config::BASIC_CONFIG.clone().into();

        AppRoutesPresenter::default()
    });

    use_context_provider(|| resource)
}
