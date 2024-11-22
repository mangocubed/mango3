use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton};
use mango3_leptos_utils::i18n::{t, use_i18n};

use crate::components::PageFormFields;
use crate::context::use_website_id_param;
use crate::server_functions::AttemptToCreatePage;

#[component]
pub fn NewPagePage() -> impl IntoView {
    let i18n = use_i18n();
    let website_id = use_website_id_param();
    let server_action = ServerAction::<AttemptToCreatePage>::new();
    let action_value = server_action.value();

    view! {
        <h3 class="h3">{t!(i18n, studio.new_page)}</h3>

        <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
            <ActionFormAlert
                action_value=action_value
                error_message=move || { t_string!(i18n, studio.failed_to_create_page) }
                redirect_to=format!("/websites/{}/pages", &website_id)
                success_message=move || { t_string!(i18n, studio.page_created_successfully) }
            />

            <input type="hidden" name="website_id" value=website_id />

            <PageFormFields action_value=action_value />

            <SubmitButton is_loading=server_action.pending() />
        </ActionForm>
    }
}
