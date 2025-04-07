use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_web_utils::async_t_string;
use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal, PasswordField, SubmitButton};
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::pages::AuthenticatedPage;
use mango3_web_utils::utils::ToSignalTrait;

use crate::server_functions::AttemptToUpdatePassword;

#[component]
pub fn ChangePasswordPage() -> impl IntoView {
    let navigate = use_navigate();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToUpdatePassword>::new();
    let action_value = server_action.value();
    let title = async_t_string!(i18n, shared.change_password).to_signal();

    view! {
        <AuthenticatedPage title=title>
            <h2 class="h2">{title}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert action_value=action_value message=move || t!(i18n, shared.failed_to_update_password) />

                <PasswordField
                    action_value=action_value
                    id="current_password"
                    label=move || t!(i18n, my_account.current_password)
                    name="current_password"
                />

                <PasswordField
                    action_value=action_value
                    id="new_password"
                    label=move || t!(i18n, shared.new_password)
                    name="new_password"
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <FormSuccessModal
                action_value=action_value
                message=move || t!(i18n, shared.password_updated_successfully)
                on_close=move || navigate("/", Default::default())
            />
        </AuthenticatedPage>
    }
}
