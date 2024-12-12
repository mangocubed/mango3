use leptos::either::Either;
use leptos::prelude::*;

use crate::components::{CurrentUserResource, UserTag};
use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};
use crate::icons::{BarsOutlined, ChevronDownMini};

#[component]
pub fn TopBar(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] brand: ViewFnOnce,
    #[prop(default = true)] show_user_menu: bool,
    #[prop(into, optional)] right_items: ViewFn,
) -> impl IntoView {
    let i18n = use_i18n();
    let children_store = StoredValue::new(children);

    let items = move || {
        view! {
            <div class="md:flex-1">{children_store.read_value().as_ref().map(|c| c())}</div>

            <div class="md:flex-none">{right_items.run()}</div>
        }
    };

    view! {
        <div class="navbar bg-base-300 shadow-md py-0 gap-2">
            <div class="dropdown md:hidden">
                <button class="btn btn-ghost" tabindex="0">
                    <BarsOutlined />
                </button>

                <div tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow w-40">
                    {items.clone()}
                </div>
            </div>

            <div class="flex-none">{brand.run()}</div>

            <div class="flex-1">
                <div class="hidden md:flex items-center w-full">{items}</div>
            </div>

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
                                                <button class="btn btn-ghost p-1 pl-2" tabindex="1">
                                                    <UserTag user=user text_class="hidden md:block" />

                                                    <ChevronDownMini />
                                                </button>

                                                <ul
                                                    tabindex="1"
                                                    class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow w-40"
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
                                                <a class="btn" href=basic_config.login_url.clone() tabindex="1">
                                                    {t!(i18n, shared.login)}
                                                </a>

                                                <ul
                                                    tabindex="1"
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
