use leptos::ev::Event;
use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;

use crate::components::ResetPasswordDialog;
use crate::server_functions::AttemptToSendPasswordResetCode;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToSendPasswordResetCode>::new();
    let action_value = server_action.value();
    let value_username_or_email = RwSignal::new(String::new());
    let error_username_or_email = RwSignal::new(None);
    let show_reset_password_dialog = RwSignal::new(false);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_username_or_email.set(response.error("username-or-email"));
    });

    let title = move || t_string!(i18n, accounts.reset_password);

    let on_input = move |event: Event| {
        value_username_or_email.set(event_target_value(&event));
    };

    view! {
        <GuestPage title=title>
            <h2 class="h2">{title}</h2>

            <ResetPasswordDialog username_or_email=value_username_or_email is_open=show_reset_password_dialog />

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t_string!(i18n, accounts.failed_to_send_password_reset_code)
                    on_success=move || show_reset_password_dialog.set(true)
                    success_message=move || { t_string!(i18n, accounts.password_reset_code_sent_successfully) }
                />

                <TextField
                    label=move || t_string!(i18n, accounts.username_or_email)
                    name="username_or_email"
                    error=error_username_or_email
                    on_input=on_input
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <div class="max-w-[640px] ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/login">
                    {t!(i18n, accounts.back_to_login)}
                </a>
            </div>
        </GuestPage>
    }
}
