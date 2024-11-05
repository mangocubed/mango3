use leptos::prelude::*;
use leptos_fluent::expect_i18n;
use leptos_use::{ColorMode, UseColorModeReturn};

use crate::context::{use_basic_config, use_color_mode};
use crate::icons::{ChevronUpMini, ComputerOutlined, MoonOutlined, SunOutlined};

#[component]
pub fn BottomBar() -> impl IntoView {
    let basic_config = use_basic_config();
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
    let i18n = expect_i18n();

    let current_lang_name = move || i18n.language.get().name;
    let available_langs = move || i18n.languages.iter().filter(|lang| !lang.is_active());

    view! {
        <footer class="footer bg-base-200 text-base-content p-10">
            <aside>
                <img class="h-[48px]" alt=basic_config.title.clone() src="/logo.svg" />
                <p>{basic_config.copyright.clone()}</p>
            </aside>

            <nav class="justify-items-end">
                <div class="dropdown dropdown-top dropdown-end">
                    <button tabindex="0" type="button" class="btn btn-outline">
                        {current_lang_name}
                        <ChevronUpMini />
                    </button>
                    <ul
                        tabindex="0"
                        class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                    >
                        <For each=available_langs key=|lang| lang.id let:lang>
                            <li>
                                <a on:click=move |_| i18n.language.set(lang)>{lang.name}</a>
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
