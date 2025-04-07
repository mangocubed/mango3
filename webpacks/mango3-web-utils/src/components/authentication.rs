use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::context::use_basic_config;
use crate::server_functions::is_authenticated;

#[component]
pub fn RequireAuthentication() -> impl IntoView {
    let is_authenticated_resource = Resource::new_blocking(|| (), |_| is_authenticated());

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                if let Some(Ok(false)) = is_authenticated_resource.get() {
                    let basic_config = use_basic_config();
                    Some(view! { <Redirect path=basic_config.login_url.clone() /> })
                } else {
                    None
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn RequireNoAuthentication() -> impl IntoView {
    let is_authenticated_resource = Resource::new_blocking(|| (), |_| is_authenticated());

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                if let Some(Ok(true)) = is_authenticated_resource.get() {
                    let basic_config = use_basic_config();
                    Some(view! { <Redirect path=basic_config.home_url.clone() /> })
                } else {
                    None
                }
            })}
        </Suspense>
    }
}
