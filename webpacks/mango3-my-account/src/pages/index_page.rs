use leptos::prelude::*;
use mango3_web_utils::async_t_string;

use mango3_web_utils::i18n::use_i18n;
use mango3_web_utils::pages::AuthenticatedPage;
use mango3_web_utils::utils::ToSignalTrait;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();

    let text_title = async_t_string!(i18n, shared.home).to_signal();

    view! {
        <AuthenticatedPage title=text_title>
            <h1 class="h1">{move || text_title.get()}</h1>
        </AuthenticatedPage>
    }
}
