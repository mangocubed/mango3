use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::forms::{ActionFormErrorAlert, ActionFormSuccessModal};
use mango3_leptos_utils::components::{PasswordField, SubmitButton, TextField};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::LoginConfirmationModal;
use crate::server_functions::AttemptToLogin;

#[component]
pub fn LoginPage() -> impl IntoView {
    let i18n = use_i18n();
    let basic_config = use_basic_config();
    let navigate = use_navigate();
    let server_action = ServerAction::<AttemptToLogin>::new();
    let action_value = server_action.value();
    let error_alert_is_active = RwSignal::new(false);
    let error_username_or_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);
    let login_confirmation_modal_is_open = RwSignal::new(false);
    let success_modal_is_open = RwSignal::new(false);
    let text_title = async_t_string!(i18n, shared.login).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        if response.is_success() {
            if response.data.as_ref().map(|user_session| user_session.is_confirmed) == Some(true) {
                success_modal_is_open.set(true);
            } else {
                login_confirmation_modal_is_open.set(true);
            }
        }

        error_alert_is_active.set(response.is_invalid());
        error_username_or_email.set(response.error("username-or-email"));
        error_password.set(response.error("password"));
    });

    view! {
        <GuestPage title=text_title>
            <h2 class="text-xl font-bold mb-4">{move || text_title.get()}</h2>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormErrorAlert
                    is_active=error_alert_is_active
                    message=move || t!(i18n, accounts.failed_to_authenticate_user)
                />

                <TextField
                    label=move || t!(i18n, accounts.username_or_email)
                    name="username_or_email"
                    error=error_username_or_email
                />

                <PasswordField label=move || t!(i18n, shared.password) name="password" error=error_password />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <LoginConfirmationModal
                is_open=login_confirmation_modal_is_open
                on_success=move || success_modal_is_open.set(true)
            />

            <ActionFormSuccessModal
                is_open=success_modal_is_open
                message=move || t!(i18n, accounts.user_authenticated_successfully)
                on_close=move || {
                    navigate(&basic_config.home_url, Default::default());
                }
            />

            <div class="max-w-[640px] ml-auto mr-auto mt-4 flex flex-col gap-4">
                <a class="btn btn-block btn-outline" href="/register">
                    {t!(i18n, accounts.i_dont_have_an_account)}
                </a>

                <a class="btn btn-block btn-outline" href="/reset-password">
                    {t!(i18n, accounts.i_forgot_my_password)}
                </a>
            </div>
        </GuestPage>
    }
}
