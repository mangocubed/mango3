use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::forms::{FormErrorAlert, FormSuccessModal, SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::FormResp;
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
    let reset_password_modal_is_open = RwSignal::new(false);
    let success_modal_is_open = RwSignal::new(false);
    let text_title = async_t_string!(i18n, accounts.reset_password).to_signal();

    Effect::new(move || {
        let response = FormResp::from(action_value);

        if response.is_success() {
            reset_password_modal_is_open.set(true);
        }
    });

    view! {
        <GuestPage title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert
                    action_value=action_value
                    message=move || t!(i18n, accounts.failed_to_send_password_reset_code)
                />

                <TextField
                    action_value=action_value
                    id="username_or_email"
                    label=move || t!(i18n, accounts.username_or_email)
                    name="username_or_email"
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <ResetPasswordModal
                is_open=reset_password_modal_is_open
                on_success=move || success_modal_is_open.set(true)
            />

            <FormSuccessModal
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
