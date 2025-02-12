use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_router::components::Redirect;

use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::pages::Page;

use crate::server_functions::is_admin;

#[component]
pub fn AdminPageContainer(
    children: Children,
    #[prop(into, optional)] class: String,
    #[prop(into)] title: TextProp,
) -> impl IntoView {
    let is_admin_resource = Resource::new_blocking(|| (), |_| is_admin());

    view! {
        <Page class=class title=title>
            <Suspense>
                {move || Suspend::new(async move {
                    if let Some(Ok(false)) = is_admin_resource.get() {
                        let basic_config = use_basic_config();
                        Some(view! { <Redirect path=basic_config.home_url /> })
                    } else {
                        None
                    }
                })}
            </Suspense>

            {children()}
        </Page>
    }
}
