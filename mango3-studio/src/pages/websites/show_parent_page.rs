use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::{AuthenticatedPage, NotFoundPage};

use crate::components::MyWebsiteOpt;
use crate::context::provide_my_website_resource;

#[component]
pub fn Submenu(show: Memo<bool>, items: Vec<(String, &'static str)>, pathname: Memo<String>) -> impl IntoView {
    if items.is_empty() {
        Either::Left(())
    } else {
        Either::Right(view! {
            <ul class="menu-dropdown" class:menu-dropdown-show=show>
                <For
                    each=move || items.clone()
                    key=|(href, _)| href.clone()
                    children=move |(href, label)| {
                        let href_clone = href.clone();
                        let is_active = move || pathname.get() == href_clone.clone();
                        view! {
                            <li>
                                <a class:active=is_active href=href>
                                    {label}
                                </a>
                            </li>
                        }
                    }
                />
            </ul>
        })
    }
}

#[component]
pub fn ShowParentPage() -> impl IntoView {
    provide_my_website_resource();

    let i18n = use_i18n();
    let location = use_location();

    view! {
        <MyWebsiteOpt children=move |website| {
            if let Some(website) = website {
                let website_name = website.name.clone();
                let home_path = format!("/websites/{}", website.id);
                let posts_path = format!("{home_path}/posts");
                let pages_path = format!("{home_path}/pages");
                let menu_items = move || [
                    (home_path.clone(), t_string!(i18n, shared.home), vec![]),
                    (
                        posts_path.clone(),
                        t_string!(i18n, shared.posts),
                        vec![(format!("{posts_path}/new"), t_string!(i18n, studio.new_post))],
                    ),
                    (
                        pages_path.clone(),
                        t_string!(i18n, studio.pages),
                        vec![(format!("{pages_path}/new"), t_string!(i18n, studio.new_page))],
                    ),
                    (format!("{home_path}/edit"), t_string!(i18n, studio.edit), vec![]),
                ];
                Either::Left(
                    view! {
                        <AuthenticatedPage title=move || {
                            format!("{} > {}", t_string!(i18n, studio.my_websites), website_name)
                        }>
                            <h2 class="h2 breadcrumbs p-0">
                                <ul>
                                    <li>
                                        <a href="/">{t!(i18n, studio.my_websites)}</a>
                                    </li>
                                    <li>{website.name}</li>
                                </ul>
                            </h2>

                            <div class="flex grow">
                                <ul class="menu bg-base-200 rounded-box w-56">
                                    <For
                                        each=menu_items
                                        key=|(href, _, _)| href.clone()
                                        children=move |(href, label, submenu_items)| {
                                            let href_clone = href.clone();
                                            let submenu_items_clone = submenu_items.clone();
                                            let is_active = Memo::new(move |_| {
                                                location.pathname.get() == href_clone.clone()
                                            });
                                            let is_dropdown = RwSignal::new(!submenu_items.is_empty());
                                            let show_submenu = Memo::new(move |_| {
                                                is_active.get()
                                                    || submenu_items_clone
                                                        .iter()
                                                        .any(|(href, _)| location.pathname.get() == *href)
                                            });
                                            view! {
                                                <li>
                                                    <a
                                                        class:active=is_active
                                                        class:menu-dropdown-toggle=is_dropdown
                                                        class:menu-dropdown-show=show_submenu
                                                        href=href
                                                    >
                                                        {label}
                                                    </a>

                                                    <Submenu
                                                        show=show_submenu
                                                        items=submenu_items
                                                        pathname=location.pathname
                                                    />
                                                </li>
                                            }
                                        }
                                    />
                                </ul>

                                <div class="grow ml-4">
                                    <Outlet />
                                </div>
                            </div>
                        </AuthenticatedPage>
                    },
                )
            } else {
                Either::Right(view! { <NotFoundPage /> })
            }
        } />
    }
}
