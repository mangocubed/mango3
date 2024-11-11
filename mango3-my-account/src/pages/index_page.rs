use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::AuthenticatedPage;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = move || t_string!(i18n, shared.home);

    view! {
        <AuthenticatedPage title=title>
            <h2 class="text-xl font-bold mb-4">{title}</h2>
        </AuthenticatedPage>
    }
}
