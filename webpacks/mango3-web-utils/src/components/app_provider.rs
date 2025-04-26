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
use crate::providers::{
    provide_app_config_resource, provide_basic_config_resource, provide_i18n, provide_info_resource,
};

#[cfg(feature = "with-dioxus")]
#[component]
pub fn AppProvider(
    #[props(optional)] class: String,
    children: Element,
    extra_locales: Option<Vec<(LanguageIdentifier, &'static str)>>,
) -> Element {
    let basic_config = provide_basic_config_resource()?;

    provide_info_resource()?;

    let app_config = provide_app_config_resource()?;

    provide_i18n(app_config.locale, extra_locales.unwrap_or_default());

    rsx! {
        document::Link { rel: "stylesheet", href: basic_config.asset_url("style.css").to_string() }

        div {
           class: format!("flex flex-col min-h-screen {class}"),
           {children}
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
