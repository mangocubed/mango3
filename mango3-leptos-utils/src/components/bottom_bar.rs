use leptos::prelude::*;
use leptos_use::{ColorMode, UseColorModeReturn};

use crate::context::{use_basic_config, use_color_mode_with_options, use_info, UseColorModeOptions};
use crate::i18n::{t, use_i18n, Locale};
use crate::icons::{ChevronUpMini, ComputerOutlined, MoonOutlined, SunOutlined};

const LANGUAGES: [(&str, Locale); 2] = [("English", Locale::en), ("Español", Locale::es)];

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
        LANGUAGES
            .iter()
            .find(|(_, locale)| *locale == i18n.get_locale())
            .unwrap()
            .0
    };
    let available_langs = move || LANGUAGES.iter().filter(move |(_, locale)| *locale != i18n.get_locale());

    view! {
        <footer class="footer bg-base-200 text-base-content p-10">
            <aside>
                <div>{aside_items.run()}</div>

                <a href=basic_config.home_url.clone()>
                    <img class="h-[48px]" alt=basic_config.title.clone() src=basic_config.asset_url("logo.svg") />
                </a>
                <p>{basic_config.copyright.clone()}</p>
            </aside>

            <nav>
                <Show when={
                    let has_about = !basic_config.about_url.is_empty();
                    move || has_about
                }>
                    <a href=basic_config.about_url.clone() target="_blank">
                        {t!(i18n, shared.about_us)}
                    </a>
                </Show>

                <Show when={
                    let has_privacy_policy = !basic_config.privacy_policy_url.is_empty();
                    move || has_privacy_policy
                }>
                    <a href=basic_config.privacy_policy_url.clone() target="_blank">
                        {t!(i18n, shared.privacy_policy)}
                    </a>
                </Show>

                <Show when={
                    let has_terms_of_service = !basic_config.terms_of_service_url.is_empty();
                    move || has_terms_of_service
                }>
                    <a href=basic_config.terms_of_service_url.clone() target="_blank">
                        {t!(i18n, shared.terms_of_service)}
                    </a>
                </Show>

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
                    <button tabindex="2" type="button" class="btn btn-outline">
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
                        class="join-item btn btn-outline"
                        class:btn-active=move || mode.get() == ColorMode::Light
                        on:click=move |_| set_mode.set(ColorMode::Light)
                    >
                        <SunOutlined />
                    </button>
                    <button
                        type="button"
                        class="join-item btn btn-outline"
                        class:btn-active=move || mode.get() == ColorMode::Dark
                        on:click=move |_| set_mode.set(ColorMode::Dark)
                    >
                        <MoonOutlined />
                    </button>
                    <button
                        type="button"
                        class="join-item btn btn-outline"
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
