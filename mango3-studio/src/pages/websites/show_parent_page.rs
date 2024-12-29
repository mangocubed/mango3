use leptos::either::Either;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use mango3_leptos_utils::i18n::{t_string, use_i18n};
use mango3_leptos_utils::icons::*;
use mango3_leptos_utils::pages::{AuthenticatedPage, NotFoundPage};

use crate::components::MyWebsiteOpt;
use crate::context::{provide_my_website_resource, use_selected_website};

#[component]
pub fn MenuItem(href: String, #[prop(into)] icon: ViewFnOnce, #[prop(into)] label: TextProp) -> impl IntoView {
    let location = use_location();
    let href_clone = href.clone();

    let is_active = move || location.pathname.get() == href_clone;
    let label_text = move || label.get();

    view! {
        <li>
            <a class:active=is_active href=href title=label_text.clone()>
                {icon.run()}
                <span class="md:inline hidden">{label_text.clone()}</span>
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
            selected_website.set(website.clone());
            if let Some(website) = website {
                let website_name = website.name.clone();
                let home_path = format!("/websites/{}", website.id);
                let posts_path = format!("{home_path}/posts");
                let pages_path = format!("{home_path}/pages");
                let navigation_path = format!("{home_path}/navigation");
                let edit_path = format!("{home_path}/edit");
                Either::Left(
                    view! {
                        <AuthenticatedPage
                            class="flex grow gap-4"
                            title=move || { format!("{} > {}", t_string!(i18n, studio.my_websites), website_name) }
                        >
                            <ul class="menu bg-base-200 rounded-box md:w-56">
                                <MenuItem
                                    href=home_path
                                    icon=move || view! { <HomeOutlined /> }
                                    label=move || t_string!(i18n, shared.home)
                                />

                                <MenuItem
                                    href=posts_path
                                    icon=move || view! { <DocumentTextOutlined /> }
                                    label=move || t_string!(i18n, shared.posts)
                                />

                                <MenuItem
                                    href=pages_path
                                    icon=move || view! { <DocumentOutlined /> }
                                    label=move || t_string!(i18n, studio.pages)
                                />

                                <MenuItem
                                    href=navigation_path
                                    icon=move || view! { <Bars3Outlined /> }
                                    label=move || t_string!(i18n, studio.navigation)
                                />

                                <MenuItem
                                    href=edit_path
                                    icon=move || view! { <PencilSquareOutlined /> }
                                    label=move || t_string!(i18n, studio.edit)
                                />
                            </ul>

                            <div class="grow">
                                <Outlet />
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
