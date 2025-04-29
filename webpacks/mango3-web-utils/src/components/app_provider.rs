#[cfg(not(feature = "with-dioxus"))]
use codee::string::FromToStringCodec;
#[cfg(feature = "with-dioxus")]
use dioxus::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(feature = "with-dioxus")]
use unic_langid::LanguageIdentifier;

#[cfg(not(feature = "with-dioxus"))]
use crate::constants::COOKIE_NAME_LANGUAGE;
#[cfg(not(feature = "with-dioxus"))]
use crate::context::{provide_basic_config, provide_current_user_resource, provide_info, use_language_cookie_options};
#[cfg(not(feature = "with-dioxus"))]
use crate::i18n::I18nContextProvider;
#[cfg(feature = "with-dioxus")]
use crate::providers::{provide_app_config_resource, provide_app_info, provide_app_routes, provide_i18n};

#[cfg(feature = "with-dioxus")]
#[component]
fn AppConfigProvider(
    children: Element,
    #[props(optional)] extra_locales: Vec<(LanguageIdentifier, &'static str)>,
) -> Element {
    let app_config_resource = provide_app_config_resource()?;
    let mut i18n = provide_i18n(extra_locales);

    use_effect(move || {
        if let Some(ref app_config) = *app_config_resource.read() {
            i18n.set_language(app_config.locale.clone());
        }
    });

    children
}

#[cfg(feature = "with-dioxus")]
#[component]
pub fn AppProvider(
    children: Element,
    #[props(optional)] class: String,
    #[props(optional)] extra_locales: Vec<(LanguageIdentifier, &'static str)>,
    favicon_href: Option<String>,
    #[props(optional)] loader_icon_class: String,
    loader_icon_url: Option<String>,
    #[props(default = "rounded-full".to_owned())] loader_pulse_class: String,
) -> Element {
    provide_app_info();
    let app_routes = provide_app_routes();
    let mut loading_is_done = use_signal(|| false);

    let favicon_href = if let Some(href) = favicon_href {
        href
    } else {
        app_routes.asset_url("favicon.ico").to_string()
    };

    let loader_icon_src = loader_icon_url
        .clone()
        .unwrap_or_else(|| app_routes.asset_url("icon.svg").to_string());

    use_effect(move || {
        *loading_is_done.write() = true;
    });

    rsx! {
        document::Link { rel: "icon", href: favicon_href }
        document::Link { rel: "stylesheet", href: app_routes.asset_url("style.css").to_string() }


        div {
           class: format!("flex flex-col min-h-screen {class}"),

           AppConfigProvider { extra_locales: extra_locales, { children } }

           div {
               class: if loading_is_done() { "loading-overlay is-done" } else { "loading-overlay" },
               figure {
                   div { class: format!("loading-pulse {loader_pulse_class}") }
                   img { class: loader_icon_class, src: loader_icon_src }
               }
           }
        }
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn AppProvider(children: Children) -> impl IntoView {
    provide_basic_config_resource();
    provide_current_user_resource();
    provide_info_resource();

    let language_cookie_options = use_language_cookie_options::<FromToStringCodec>();

    view! {
        <I18nContextProvider cookie_name=COOKIE_NAME_LANGUAGE cookie_options=language_cookie_options>
            <div class="flex flex-col min-h-screen">{children()}</div>
        </I18nContextProvider>
    }
}
