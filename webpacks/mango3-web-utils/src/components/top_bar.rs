use std::sync::Arc;

use leptos::either::Either;
use leptos::prelude::*;

use crate::components::{CurrentUserOpt, UserTag};
use crate::context::use_basic_config;
use crate::enums::Orientation;
use crate::i18n::{t, use_i18n};
use crate::icons::{Bars3Outlined, ChevronDownMini};

#[derive(Clone)]
pub struct ItemsViewFn(Arc<dyn Fn(Orientation) -> AnyView + Send + Sync + 'static>);

impl Default for ItemsViewFn {
    fn default() -> Self {
        Self(Arc::new(|_| ().into_any()))
    }
}

impl<F, C> From<F> for ItemsViewFn
where
    F: Fn(Orientation) -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(value: F) -> Self {
        Self(Arc::new(move |orientation| value(orientation).into_any()))
    }
}

#[component]
pub fn TopBar(
    #[prop(into)] brand: ViewFnOnce,
    #[prop(default = "bg-base-300")] class: &'static str,
    #[prop(into, optional)] left_items: ItemsViewFn,
    #[prop(into, optional)] right_items: ItemsViewFn,
    #[prop(default = true)] show_user_menu: bool,
) -> impl IntoView {
    let i18n = use_i18n();
    let left_items_store = StoredValue::new(left_items);
    let right_items_store = StoredValue::new(right_items);

    view! {
        <div class=format!("navbar shadow-md py-0 gap-2 {class}")>
            <div class="dropdown md:hidden">
                <button class="btn btn-ghost btn-lg" tabindex="0">
                    <Bars3Outlined />
                </button>

                <div tabindex="0" class="dropdown-content menu bg-base-100 rounded-box z-[1] p-2 shadow w-65">
                    <div class="max-w-full">
                        {left_items_store.with_value(|left_items| left_items.0(Orientation::Vertical))}
                    </div>

                    <div>{right_items_store.with_value(|right_items| right_items.0(Orientation::Vertical))}</div>
                </div>
            </div>

            <div class="flex-none">{brand.run()}</div>

            <div class="flex-1">
                <div class="hidden md:flex items-center w-full">
                    <div class="flex-1 max-w-full">
                        {left_items_store.with_value(|left_items| left_items.0(Orientation::Horizontal))}
                    </div>

                    <div class="flex-none">
                        {right_items_store.with_value(|right_items| right_items.0(Orientation::Horizontal))}
                    </div>
                </div>
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
                                    let can_insert_website = user.can_insert_website;
                                    let new_website_url = basic_config.new_website_url.clone();
                                    let studio_url = basic_config.studio_url.to_string();
                                    let my_account_url = basic_config.my_account_url.to_string();
                                    Either::Left(
                                        view! {
                                            <div class="dropdown dropdown-end">
                                                <button class="btn btn-ghost btn-lg px-2" tabindex="1">
                                                    <UserTag user=user.clone() text_class="hidden md:block text-xs" />

                                                    <ChevronDownMini />
                                                </button>

                                                <ul
                                                    tabindex="1"
                                                    class="dropdown-content menu bg-base-200 rounded-box z-[1] p-2 shadow w-48"
                                                >
                                                    <li>
                                                        <a href=user.url.to_string()>{t!(i18n, shared.profile)}</a>
                                                    </li>

                                                    <Show when=move || {
                                                        can_insert_website
                                                    }>
                                                        {
                                                            let new_website_url = new_website_url.clone();
                                                            move || {
                                                                view! {
                                                                    <li>
                                                                        <a href=new_website_url
                                                                            .to_string()>{t!(i18n, shared.new_website)}</a>
                                                                    </li>
                                                                }
                                                            }
                                                        }
                                                    </Show>

                                                    <li>
                                                        <a href=studio_url>{t!(i18n, shared.studio)}</a>
                                                    </li>

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
                                            <a
                                                class="btn btn-ghost btn-block px-2"
                                                href=basic_config.login_url.to_string()
                                                tabindex="1"
                                            >
                                                {t!(i18n, shared.login)}
                                            </a>
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
