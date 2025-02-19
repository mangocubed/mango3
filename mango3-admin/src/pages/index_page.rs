use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::AdminPageContainer;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();
    let text_title = async_t_string!(i18n, shared.home).to_signal();

    view! {
        <AdminPageContainer title=text_title>
            <h1 class="h1">{move || text_title.get()}</h1>
        </AdminPageContainer>
    }
}
