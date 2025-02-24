use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::forms::{ActionFormErrorAlert, ActionFormSuccessModal};
use mango3_leptos_utils::components::{SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::ResetPasswordModal;
use crate::server_functions::AttemptToSendPasswordResetCode;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    let i18n = use_i18n();
    let navigate = use_navigate();
    let server_action = ServerAction::<AttemptToSendPasswordResetCode>::new();
    let action_value = server_action.value();
    let error_alert_is_active = RwSignal::new(false);
    let error_username_or_email = RwSignal::new(None);
    let reset_password_modal_is_open = RwSignal::new(false);
    let success_modal_is_open = RwSignal::new(false);
    let text_title = async_t_string!(i18n, accounts.reset_password).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        if response.is_success() {
            reset_password_modal_is_open.set(true);
        }

        error_alert_is_active.set(response.is_invalid());
        error_username_or_email.set(response.error("username-or-email"));
    });

    view! {
        <GuestPage title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormErrorAlert
                    is_active=error_alert_is_active
                    message=move || t!(i18n, accounts.failed_to_send_password_reset_code)
                />

                <TextField
                    label=move || t!(i18n, accounts.username_or_email)
                    name="username_or_email"
                    error=error_username_or_email
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <ResetPasswordModal
                is_open=reset_password_modal_is_open
                on_success=move || success_modal_is_open.set(true)
            />

            <ActionFormSuccessModal
                is_open=success_modal_is_open
                message=move || t!(i18n, shared.password_updated_successfully)
                on_close=move || {
                    navigate("/login", Default::default());
                }
            />

            <div class="max-w-[640px] ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/login">
                    {t!(i18n, accounts.back_to_login)}
                </a>
            </div>
        </GuestPage>
    }
}
