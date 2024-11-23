use leptos::prelude::*;
use leptos_use::{ColorMode, UseColorModeReturn};

use crate::context::{use_basic_config, use_color_mode};
use crate::i18n::{use_i18n, Locale};
use crate::icons::{ChevronUpMini, ComputerOutlined, MoonOutlined, SunOutlined};

const LANGUAGES: [(&str, Locale); 2] = [("English", Locale::en), ("Español", Locale::es)];

#[component]
pub fn BottomBar() -> impl IntoView {
    let basic_config = use_basic_config();
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
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
                <a href=basic_config.home_url.clone()>
                    <img class="h-[48px]" alt=basic_config.title.clone() src=basic_config.asset_url("logo.svg") />
                </a>
                <p>{basic_config.copyright.clone()}</p>
            </aside>

            <nav class="justify-items-end">
                <div class="dropdown dropdown-top dropdown-end">
                    <button tabindex="0" type="button" class="btn btn-outline">
                        {current_lang_name}
                        <ChevronUpMini />
                    </button>
                    <ul tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow">
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
