use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton, TextField, TextareaField};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::AuthenticatedPage;

use crate::server_functions::AttemptToCreateWebsite;

#[component]
pub fn NewWebsitePage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToCreateWebsite>::new();
    let action_value = server_action.value();
    let error_name = RwSignal::new(None);
    let error_subdomain = RwSignal::new(None);
    let error_description = RwSignal::new(None);
    let value_subdomain = RwSignal::new("".to_owned());

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_name.set(response.error("name"));
        error_subdomain.set(response.error("subdomain"));
        error_description.set(response.error("description"));
    });

    let name_on_input = move |event| {
        value_subdomain.set(slug::slugify(event_target_value(&event)));
    };

    let title = move || t_string!(i18n, shared.new_website);

    view! {
        <AuthenticatedPage title=title>
            <h2 class="text-xl font-bold mb-4">{title}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t_string!(i18n, studio.failed_to_create_website)
                    redirect_to="/"
                    success_message=move || t_string!(i18n, studio.website_created_successfully)
                />

                <TextField
                    label=move || t_string!(i18n, studio.name)
                    name="name"
                    error=error_name
                    on_input=name_on_input
                />

                <TextField
                    label=move || t_string!(i18n, studio.subdomain)
                    name="subdomain"
                    value=value_subdomain
                    error=error_subdomain
                />

                <TextareaField
                    label=move || t_string!(i18n, studio.description)
                    name="description"
                    error=error_description
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </AuthenticatedPage>
    }
}
