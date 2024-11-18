use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, PasswordField, SubmitButton};
use mango3_leptos_utils::i18n::use_i18n;
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::AuthenticatedPage;

use crate::server_functions::AttemptToUpdatePassword;

#[component]
pub fn ChangePasswordPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToUpdatePassword>::new();
    let action_value = server_action.value();
    let error_current_password = RwSignal::new(None);
    let error_new_password = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_current_password.set(response.error("current-password"));
        error_new_password.set(response.error("new-password"));
    });

    let title = move || t_string!(i18n, shared.change_password);

    view! {
        <AuthenticatedPage title=title>
            <h2 class="h2">{title}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t_string!(i18n, shared.failed_to_update_password)
                    redirect_to="/"
                    success_message=move || { t_string!(i18n, shared.password_updated_successfully) }
                />

                <PasswordField
                    label=move || t_string!(i18n, my_account.current_password)
                    name="current_password"
                    error=error_current_password
                />

                <PasswordField
                    label=move || t_string!(i18n, shared.new_password)
                    name="new_password"
                    error=error_new_password
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </AuthenticatedPage>
    }
}
