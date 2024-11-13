use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::{AuthenticatedPage, NotFoundPage};

use crate::components::MyWebsiteResource;
use crate::context::provide_my_website_resource;

#[component]
pub fn ShowParentPage() -> impl IntoView {
    provide_my_website_resource();

    let i18n = use_i18n();
    let location = use_location();

    view! {
        <MyWebsiteResource children=move |website| {
            if let Some(website) = website {
                let website_name = website.name.clone();
                let edit_path = format!("/websites/{}/edit", website.id);
                Either::Left(
                    view! {
                        <AuthenticatedPage title=move || {
                            format!("{} > {}", t_string!(i18n, studio.my_websites), website_name)
                        }>
                            <h2 class="text-lg font-bold mb-4 breadcrumbs p-0">
                                <ul>
                                    <li>
                                        <a href="/">{t!(i18n, studio.my_websites)}</a>
                                    </li>
                                    <li>{website.name}</li>
                                </ul>
                            </h2>

                            <div class="flex grow">
                                <ul class="menu bg-base-200 rounded-box w-56">
                                    <li>
                                        <a
                                            class:active={
                                                let edit_path = edit_path.clone();
                                                move || { location.pathname.get() == edit_path }
                                            }
                                            href=edit_path
                                        >
                                            {t!(i18n, studio.edit)}
                                        </a>
                                    </li>
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
