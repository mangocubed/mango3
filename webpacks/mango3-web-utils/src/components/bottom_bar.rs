#[cfg(feature = "with-dioxus")]
use dioxus::prelude::*;
#[cfg(feature = "with-dioxus")]
use dioxus_i18n::t;
#[cfg(not(feature = "with-dioxus"))]
use leptos::prelude::*;
#[cfg(not(feature = "with-dioxus"))]
use leptos_use::{ColorMode, UseColorModeReturn};
#[cfg(feature = "with-dioxus")]
use unic_langid::{langid, LanguageIdentifier};

use crate::icons::ChevronUpMini;

#[cfg(not(feature = "with-dioxus"))]
use crate::context::{use_basic_config, use_color_mode_with_options, use_info, UseColorModeOptions};
#[cfg(feature = "with-dioxus")]
use crate::hooks::{use_app_config_resource, use_routesS, use_info};
#[cfg(not(feature = "with-dioxus"))]
use crate::i18n::{t, use_i18n, Locale};
#[cfg(not(feature = "with-dioxus"))]
use crate::icons::{ComputerOutlined, MoonOutlined, SunOutlined};
#[cfg(feature = "with-dioxus")]
use crate::server_functions::set_language;

#[cfg(feature = "with-dioxus")]
const LOCALES: [(&str, LanguageIdentifier); 3] = [
    ("English", langid!("en")),
    ("Español", langid!("es")),
    ("Português", langid!("pt")),
];

#[cfg(not(feature = "with-dioxus"))]
const LANGUAGES: [(&str, Locale); 3] = [
    ("English", Locale::en),
    ("Español", Locale::es),
    ("Português", Locale::pt),
];

#[cfg(feature = "with-dioxus")]
#[component]
pub fn BottomBar(
    aside_items: Option<Element>,
    #[props(default = "light".to_owned())] light_theme: String,
    #[props(default = "dark".to_owned())] dark_theme: String,
) -> Element {
    let mut app_config_resource = use_app_config_resource();
    let current_locale = use_memo(|| app_config_resource.read().as_ref().unwrap_or(LOCALES[0]));
    
    let routes = use_routes();
    let info = use_info();

    let available_locales = LOCALES
        .iter()
        .filter(move |(_, locale)| locale.language != current_locale.language)
        .cloned()
        .collect::<Vec<(&str, LanguageIdentifier)>>();

    rsx! {
        footer {
            class: "footer md:footer-horizontal bg-base-200 text-base-content p-10",
            aside {
                div { { aside_items } }

                Link {
                    to: routes.home_url.to_string(),
                    img {
                        class: "h-[48px]",
                        alt: routes.title.clone(),
                        src: routes.asset_url("logo.svg").to_string()
                    }
                }

                p { { info.copyright.clone() } }
            }

            nav {
                { routes.about_url.clone().map(|url| rsx! { a { href: url.to_string(), target: "_blank", { t!("about-us") } } }) }

                { routes.privacy_policy_url.clone().map(|url| rsx! { a { href: url.to_string(), target: "_blank", { t!("privacy-policy") } } }) }

                { routes.terms_of_service_url.clone().map(|url| rsx! { a { href: url.to_string(), target: "_blank", { t!("terms-of-service") } } }) }

                a {
                    href: format!("https://github.com/mangocubed/mango3/tree/{}", info.git_commit_hash),
                    target: "_blank",
                    title: t!("view-source-code"),
                    "v"
                    {info.version}
                    " ("
                    {info.git_commit_short_hash}
                    ")"
                }
            }

            nav {
                div {
                    class: "dropdown dropdown-top",
                    button {
                        class: "btn btn-outline btn-accent",
                        type: "button",
                        { current_locale.0 }
                        ChevronUpMini {}
                    }


                    ul {
                        class: "dropdown-content menu bg-base-100 rounded-box z-[1] w-28 p-2 shadow",
                        for (label, locale) in available_locales {
                            li {
                                key: locale.to_string(),
                                a {
                                    onclick: move |_| {
                                        let locale = locale.clone();
                                        async move { let _ = set_language(locale).await; app_config_resource.restart(); }
                                    },
                                    { label }
                                }
                            }
                        }
                    }
                }
                
                div {
                    class: "join",
                    button {
                        class="join-item btn btn-outline btn-accent",
                        type: "button",
                    }
                    
                    button {
                        class="join-item btn btn-outline btn-accent",
                        type: "button",
                    }
                    
                    button {
                        class="join-item btn btn-outline btn-accent",
                        type: "button",
                    }
                }
            }
        }
    }
}

#[cfg(not(feature = "with-dioxus"))]
#[component]
pub fn BottomBar(
    #[prop(optional, into)] aside_items: ViewFnOnce,
    #[prop(default = "light".to_owned(), into)] light_theme: String,
    #[prop(default = "dark".to_owned(), into)] dark_theme: String,
) -> impl IntoView {
    let basic_config = use_basic_config();
    let info = use_info();
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .light_theme(light_theme)
            .dark_theme(dark_theme),
    );
    let i18n = use_i18n();

    let current_lang_name = move || {
        LOCALES
            .iter()
            .find(|(_, locale)| *locale == i18n.get_locale())
            .unwrap()
            .0
    };
    let available_langs = move || LOCALES.iter().filter(move |(_, locale)| *locale != i18n.get_locale());

    view! {
        <footer class="footer md:footer-horizontal bg-base-200 text-base-content p-10">
            <aside>
                <div>{aside_items.run()}</div>

                <a href=basic_config.home_url.to_string()>
                    <img
                        class="h-[48px]"
                        alt=basic_config.title.clone()
                        src=basic_config.asset_url("logo.svg").to_string()
                    />
                </a>
                <p>{basic_config.copyright.clone()}</p>
            </aside>

            <nav>
                {move || {
                    basic_config
                        .about_url
                        .clone()
                        .map(|about_url| {
                            view! {
                                <a href=about_url.to_string() target="_blank">
                                    {t!(i18n, shared.about_us)}
                                </a>
                            }
                        })
                }}
                {move || {
                    basic_config
                        .privacy_policy_url
                        .clone()
                        .map(|privacy_policy_url| {
                            view! {
                                <a href=privacy_policy_url.to_string() target="_blank">
                                    {t!(i18n, shared.privacy_policy)}
                                </a>
                            }
                        })
                }}
                {move || {
                    basic_config
                        .terms_of_service_url
                        .clone()
                        .map(|terms_of_service_url| {
                            view! {
                                <a href=terms_of_service_url.to_string() target="_blank">
                                    {t!(i18n, shared.terms_of_service)}
                                </a>
                            }
                        })
                }}
                <a
                    href=format!("https://github.com/mangocubed/mango3/tree/{}", info.git_commit_hash)
                    target="_blank"
                    title=move || async_t_string!(i18n, shared.view_source_code).get()
                >
                    "v"
                    {info.version}
                    " ("
                    {info.git_commit_short_hash}
                    ")"
                </a>
            </nav>

            <nav>
                <div class="dropdown dropdown-top">
                    <button tabindex="2" type="button" class="btn btn-outline btn-accent">
                        {current_lang_name}
                        <ChevronUpMini />
                    </button>
                    <ul tabindex="2" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-28 p-2 shadow">
                        <For each=available_langs key=|lang| lang.0 let:lang>
                            <li>
                                <a on:click=move |_| i18n.set_locale(lang.1)>{lang.0}</a>
                            </li>
                        </For>
                    </ul>
                </div>

                <div class="join">
                    <button
                        type="button"
                        class="join-item btn btn-outline btn-accent"
                        class:btn-active=move || mode.get() == ColorMode::Light
                        on:click=move |_| set_mode.set(ColorMode::Light)
                    >
                        <SunOutlined />
                    </button>
                    <button
                        type="button"
                        class="join-item btn btn-outline btn-accent"
                        class:btn-active=move || mode.get() == ColorMode::Dark
                        on:click=move |_| set_mode.set(ColorMode::Dark)
                    >
                        <MoonOutlined />
                    </button>
                    <button
                        type="button"
                        class="join-item btn btn-outline btn-accent"
                        class:btn-active=move || mode.get() == ColorMode::Auto
                        on:click=move |_| set_mode.set(ColorMode::Auto)
                    >
                        <ComputerOutlined />
                    </button>
                </div>
            </nav>
        </footer>
    }
}
