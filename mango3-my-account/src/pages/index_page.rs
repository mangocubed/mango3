use leptos::prelude::*;
use mango3_leptos_utils::async_t_string;

use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();

    let text_title = async_t_string!(i18n, shared.home).to_signal();

    view! {
        <AuthenticatedPage title=text_title>
            <h2 class="text-xl font-bold mb-4">{move || text_title.get()}</h2>
        </AuthenticatedPage>
    }
}
