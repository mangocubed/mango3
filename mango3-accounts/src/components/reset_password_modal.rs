use leptos::prelude::*;

use mango3_leptos_utils::components::forms::ActionFormErrorAlert;
use mango3_leptos_utils::components::{Modal, PasswordField, SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::InformationCircleOutlined;
use mango3_leptos_utils::models::ActionFormResp;

use crate::server_functions::AttemptToResetPassword;

#[component]
pub fn ResetPasswordModal(is_open: RwSignal<bool>, #[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToResetPassword>::new();
    let action_value = server_action.value();
    let error_alert_is_active = RwSignal::new(false);
    let error_code = RwSignal::new(None);
    let error_new_password = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        if response.is_success() {
            is_open.set(false);
            on_success.run(());
        }

        error_alert_is_active.set(response.is_invalid());
        error_code.set(response.error("code"));
        error_new_password.set(response.error("new-password"));
    });

    view! {
        <Modal is_open=is_open>
            <h4 class="text-lg font-bold">{t!(i18n, shared.change_password)}</h4>

            <div role="alert" class="alert mt-4">
                <InformationCircleOutlined class="self-start my-2" />

                <div>{t!(i18n, accounts.a_confirmation_code_has_been_sent_to_your_email_address)}"."</div>
            </div>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormErrorAlert
                    is_active=error_alert_is_active
                    message=move || t!(i18n, shared.failed_to_update_password)
                />

                <TextField label=move || t!(i18n, shared.code) name="code" error=error_code />

                <PasswordField
                    label=move || t!(i18n, shared.new_password)
                    name="new_password"
                    error=error_new_password
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Modal>
    }
}
