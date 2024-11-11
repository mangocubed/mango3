use leptos::either::Either;
use leptos::prelude::*;

use crate::components::CurrentUserResource;
use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};
use crate::icons::ChevronDownMini;

#[component]
pub fn TopBar(
    children: Children,
    #[prop(default = true)] show_user_menu: bool,
    #[prop(optional, into)] right_items: ViewFnOnce,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="navbar bg-base-300 shadow-md min-h-[52px] h-[52px]">
            <div class="flex-1">{children()}</div>

            <div class="flex-none">{right_items.run()}</div>

            <Show when=move || {
                show_user_menu
            }>
                {move || {
                    let basic_config = use_basic_config();
                    view! {
                        <div class="flex-none">
                            <CurrentUserResource children=move |user| {
                                if let Some(user) = user {
                                    Either::Left(
                                        view! {
                                            <div class="dropdown dropdown-end">
                                                <button class="btn" tabindex="0">
                                                    <div class="avatar placeholder">
                                                        <div class="bg-neutral text-neutral-content w-8 rounded-full">
                                                            <span class="text-xs">{user.initials}</span>
                                                        </div>
                                                    </div>

                                                    <div>
                                                        <div class="mb-1">{user.display_name}</div>
                                                        <div class="font-bold">"@"{user.username}</div>
                                                    </div>

                                                    <ChevronDownMini />
                                                </button>

                                                <ul
                                                    tabindex="0"
                                                    class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow"
                                                >
                                                    <li>
                                                        <a href=basic_config
                                                            .new_website_url
                                                            .clone()>{t!(i18n, shared.new_website)}</a>
                                                    </li>
                                                    <li>
                                                        <a href=basic_config
                                                            .studio_url
                                                            .clone()>{t!(i18n, shared.studio)}</a>
                                                    </li>
                                                    <li>
                                                        <a href=basic_config
                                                            .my_account_url
                                                            .clone()>{t!(i18n, shared.my_account)}</a>
                                                    </li>
                                                </ul>
                                            </div>
                                        },
                                    )
                                } else {
                                    Either::Right(
                                        view! {
                                            <div class="dropdown dropdown-end dropdown-hover">
                                                <a
                                                    class="btn"
                                                    href=basic_config.login_url.clone()
                                                    tabindex="0"
                                                >
                                                    {t!(i18n, shared.login)}
                                                </a>

                                                <ul
                                                    tabindex="0"
                                                    class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow"
                                                >
                                                    <li>
                                                        <a href=basic_config
                                                            .register_url
                                                            .clone()>{t!(i18n, shared.register)}</a>
                                                    </li>
                                                </ul>
                                            </div>
                                        },
                                    )
                                }
                            } />
                        </div>
                    }
                }}
            </Show>
        </div>
    }
}
