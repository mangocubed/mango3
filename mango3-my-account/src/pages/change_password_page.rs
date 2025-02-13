use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{ActionFormAlert, PasswordField, SubmitButton};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::server_functions::AttemptToUpdatePassword;

#[component]
pub fn ChangePasswordPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToUpdatePassword>::new();
    let action_value = server_action.value();
    let error_current_password = RwSignal::new(None);
    let error_new_password = RwSignal::new(None);
    let title = async_t_string!(i18n, shared.change_password).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_current_password.set(response.error("current-password"));
        error_new_password.set(response.error("new-password"));
    });

    view! {
        <AuthenticatedPage title=title>
            <h2 class="h2">{title}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormAlert
                    action_value=action_value
                    error_message=async_t_string!(i18n, shared.failed_to_update_password).to_signal()
                    redirect_to="/"
                    success_message=async_t_string!(i18n, shared.password_updated_successfully).to_signal()
                />

                <PasswordField
                    label=async_t_string!(i18n, my_account.current_password).to_signal()
                    name="current_password"
                    error=error_current_password
                />

                <PasswordField
                    label=async_t_string!(i18n, shared.new_password).to_signal()
                    name="new_password"
                    error=error_new_password
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </AuthenticatedPage>
    }
}
