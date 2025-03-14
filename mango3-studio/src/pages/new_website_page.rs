use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::forms::{
    FormErrorAlert, FormSuccessModal, MarkdownEditorField, SubmitButton, TextField,
};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::context::use_selected_website;
use crate::server_functions::AttemptToCreateWebsite;

#[component]
pub fn NewWebsitePage() -> impl IntoView {
    let navigate = use_navigate();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToCreateWebsite>::new();
    let action_value = server_action.value();
    let value_subdomain = RwSignal::new("".to_owned());
    let title = async_t_string!(i18n, shared.new_website).to_signal();

    let selected_website = use_selected_website();

    selected_website.set(None);

    let name_on_input = move |event| {
        value_subdomain.set(slug::slugify(event_target_value(&event)));
    };

    view! {
        <AuthenticatedPage title=title>
            <h2 class="text-xl font-bold mb-4">{title}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert action_value=action_value message=move || t!(i18n, studio.failed_to_create_website) />

                <TextField
                    action_value=action_value
                    id="name"
                    label=move || t!(i18n, studio.name)
                    name="name"
                    on_input=name_on_input
                />

                <TextField
                    action_value=action_value
                    id="subdomain"
                    label=move || t!(i18n, studio.subdomain)
                    name="subdomain"
                    value=value_subdomain
                />

                <MarkdownEditorField
                    action_value=action_value
                    id="description"
                    label=move || t!(i18n, studio.description)
                    name="description"
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <FormSuccessModal
                action_value=action_value
                message=move || t!(i18n, studio.website_created_successfully)
                on_close=move || {
                    navigate("/", Default::default());
                }
            />
        </AuthenticatedPage>
    }
}
