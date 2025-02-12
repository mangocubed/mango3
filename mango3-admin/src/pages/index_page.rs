use leptos::prelude::*;

use mango3_leptos_utils::i18n::{t, t_string, use_i18n};

use crate::components::AdminPageContainer;

#[component]
pub fn IndexPage() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <AdminPageContainer title=move || t_string!(i18n, shared.home)>
            <h1 class="h1">{t!(i18n, shared.home)}</h1>
        </AdminPageContainer>
    }
}
