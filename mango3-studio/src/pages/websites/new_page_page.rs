use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_router::hooks::use_params_map;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton};
use mango3_leptos_utils::i18n::{t, use_i18n};

use crate::components::PageFormFields;
use crate::context::param_website_id;
use crate::server_functions::AttemptToCreatePage;

#[component]
pub fn NewPagePage() -> impl IntoView {
    let i18n = use_i18n();
    let params_map = use_params_map();
    let server_action = ServerAction::<AttemptToCreatePage>::new();
    let action_value = server_action.value();

    view! {
        <h2 class="h2">{t!(i18n, studio.new_page)}</h2>

        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
            <ActionFormAlert
                action_value=action_value
                error_message=move || { t_string!(i18n, studio.failed_to_create_page) }
                redirect_to=format!("/websites/{}/pages", param_website_id(params_map).unwrap_or_default())
                success_message=move || { t_string!(i18n, studio.page_created_successfully) }
            />

            <input type="hidden" name="website_id" value=move || param_website_id(params_map) />

            <PageFormFields action_value=action_value />

            <SubmitButton is_loading=server_action.pending() />
        </ActionForm>
    }
}
