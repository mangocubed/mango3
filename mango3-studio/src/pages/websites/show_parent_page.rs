use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::icons::*;
use mango3_leptos_utils::pages::NotFoundPage;

use crate::components::MyWebsiteOpt;
use crate::context::{provide_my_website_resource, use_selected_website};

#[component]
pub fn MenuItem(
    href: String,
    #[prop(into)] icon: ViewFnOnce,
    #[prop(into)] label: AsyncDerived<&'static str>,
) -> impl IntoView {
    let location = use_location();
    let href_clone = href.clone();

    let is_active = move || location.pathname.get() == href_clone;
    let label_text = move || label.get();

    view! {
        <li>
            <a class:menu-active=is_active href=href title=label_text>
                {icon.run()}
                <span class="md:inline hidden">{label_text}</span>
            </a>
        </li>
    }
}

#[component]
pub fn ShowParentPage() -> impl IntoView {
    provide_my_website_resource();

    let i18n = use_i18n();

    view! {
        <MyWebsiteOpt children=move |website| {
            let selected_website = use_selected_website();
            selected_website.set(website.as_ref().map(|w| w.into()));
            if let Some(website) = website {
                let home_path = format!("/websites/{}", website.id);
                let posts_path = format!("{home_path}/posts");
                let files_path = format!("{home_path}/files");
                let navigation_path = format!("{home_path}/navigation");
                let edit_path = format!("{home_path}/edit");
                Either::Left(
                    view! {
                        <div class="flex grow gap-4">
                            <ul class="menu bg-base-200 rounded-box md:w-56">
                                <MenuItem
                                    href=home_path
                                    icon=move || view! { <HomeOutlined /> }
                                    label=async_t_string!(i18n, shared.home)
                                />

                                <MenuItem
                                    href=posts_path
                                    icon=move || view! { <DocumentTextOutlined /> }
                                    label=async_t_string!(i18n, shared.posts)
                                />

                                <MenuItem
                                    href=files_path
                                    icon=move || view! { <PaperClipOutlined /> }
                                    label=async_t_string!(i18n, studio.files)
                                />

                                <MenuItem
                                    href=navigation_path
                                    icon=move || view! { <Bars3Outlined /> }
                                    label=async_t_string!(i18n, studio.navigation)
                                />

                                <MenuItem
                                    href=edit_path
                                    icon=move || view! { <PencilSquareOutlined /> }
                                    label=async_t_string!(i18n, studio.edit)
                                />
                            </ul>

                            <div class="grow">
                                <Outlet />
                            </div>
                        </div>
                    },
                )
            } else {
                Either::Right(NotFoundPage)
            }
        } />
    }
}
