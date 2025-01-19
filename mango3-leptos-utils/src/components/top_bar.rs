use leptos::either::Either;
use leptos::prelude::*;

use crate::components::{CurrentUserOpt, UserTag};
use crate::context::use_basic_config;
use crate::i18n::{t, use_i18n};
use crate::icons::{Bars3Outlined, ChevronDownMini};

#[component]
pub fn TopBar(
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into)] brand: ViewFnOnce,
    #[prop(default = "bg-base-300")] class: &'static str,
    #[prop(default = true)] show_user_menu: bool,
    #[prop(into, optional)] right_items: ViewFn,
) -> impl IntoView {
    let i18n = use_i18n();
    let children_store = StoredValue::new(children);

    let items = move || {
        view! {
            <div class="md:flex-1 max-w-full">{children_store.read_value().as_ref().map(|c| c())}</div>

            <div class="md:flex-none">{right_items.run()}</div>
        }
    };

    view! {
        <div class=format!("navbar shadow-md py-0 gap-2 {class}")>
            <div class="dropdown md:hidden">
                <button class="btn btn-ghost" tabindex="0">
                    <Bars3Outlined />
                </button>

                <div tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow w-72">
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
                            <CurrentUserOpt children=move |user| {
                                if let Some(user) = user {
                                    let is_creator = user.is_creator;
                                    let new_website_url = basic_config.new_website_url.clone();
                                    let studio_url = basic_config.studio_url.clone();
                                    let my_account_url = basic_config.my_account_url.clone();
                                    Either::Left(
                                        view! {
                                            <div class="dropdown dropdown-end">
                                                <button class="btn btn-ghost px-2" tabindex="1">
                                                    <UserTag user=user.clone() text_class="hidden md:block" />

                                                    <ChevronDownMini />
                                                </button>

                                                <ul
                                                    tabindex="1"
                                                    class="dropdown-content menu bg-base-200 rounded-box z-[1] p-2 shadow w-48"
                                                >

                                                    <Show when=move || {
                                                        is_creator
                                                    }>
                                                        {
                                                            let new_website_url = new_website_url.clone();
                                                            let studio_url = studio_url.clone();
                                                            move || {
                                                                let new_website_url = new_website_url.clone();
                                                                let studio_url = studio_url.clone();
                                                                view! {
                                                                    <li>
                                                                        <a href=new_website_url>{t!(i18n, shared.new_website)}</a>
                                                                    </li>

                                                                    <li>
                                                                        <a href=studio_url>{t!(i18n, shared.studio)}</a>
                                                                    </li>
                                                                }
                                                            }
                                                        }
                                                    </Show>

                                                    <li>
                                                        <a href=my_account_url>{t!(i18n, shared.my_account)}</a>
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
