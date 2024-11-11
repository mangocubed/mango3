use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_i18n::t_string;

use leptos_router::hooks::use_params_map;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::{AuthenticatedPage, NotFoundPage};

use crate::constants::KEY_PARAM_WEBSITE_ID;
use crate::server_functions::get_my_website;

#[component]
pub fn ShowPage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let website_id = params_map.with(|params| params.get(KEY_PARAM_WEBSITE_ID).unwrap_or_default());
    let my_website_resource = Resource::new_blocking(move || website_id.clone(), get_my_website);

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="flex">
                    <span class="loading loading-spinner loading-lg m-auto"></span>
                </div>
            }
        }>
            {move || Suspend::new(async move {
                match my_website_resource.get() {
                    Some(Ok(Some(website))) => {
                        let website_name = website.name.clone();
                        EitherOf3::A(
                            view! {
                                <AuthenticatedPage title=move || {
                                    format!(
                                        "{} > {}",
                                        t_string!(i18n, studio.my_websites),
                                        website_name,
                                    )
                                }>
                                    <h2 class="text-xl font-bold mb-4 breadcrumbs">
                                        <ul>
                                            <li>
                                                <a href="/">{t!(i18n, studio.my_websites)}</a>
                                            </li>
                                            <li>{website.name}</li>
                                        </ul>
                                    </h2>
                                </AuthenticatedPage>
                            },
                        )
                    }
                    Some(Ok(None)) => EitherOf3::B(view! { <NotFoundPage /> }),
                    _ => EitherOf3::C(()),
                }
            })}
        </Suspense>
    }
}
