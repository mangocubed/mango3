use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{ActionFormAlert, CurrentUser, PasswordField, SubmitButton, TextField};
use mango3_leptos_utils::context::use_current_user_resource;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::AuthenticatedPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::EmailConfirmationBadge;
use crate::server_functions::AttemptToUpdateEmail;

#[component]
pub fn EditEmailPage() -> impl IntoView {
    let i18n = use_i18n();
    let current_user_resource = use_current_user_resource();
    let server_action = ServerAction::<AttemptToUpdateEmail>::new();
    let action_value = server_action.value();
    let error_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);
    let text_title = async_t_string!(i18n, my_account.edit_email).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_email.set(response.error("email"));
        error_password.set(response.error("password"));
    });

    view! {
        <AuthenticatedPage title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <section class="max-w-[640px] w-full ml-auto mr-auto">
                <h3 class="h3">{t!(i18n, my_account.current_email)}</h3>

                <div class="flex items-center justify-between">
                    <CurrentUser children=move |user| {
                        let email_is_confirmed = RwSignal::new(user.email_is_confirmed);
                        view! {
                            <span>{user.email}</span>

                            <EmailConfirmationBadge is_confirmed=email_is_confirmed />
                        }
                    } />
                </div>
            </section>

            <section class="max-w-[640px] w-full ml-auto mr-auto mt-4">
                <h3 class="h3">{t!(i18n, my_account.change_email)}</h3>

                <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                    <ActionFormAlert
                        action_value=action_value
                        error_message=async_t_string!(i18n, my_account.failed_to_update_email).to_signal()
                        on_success=move || current_user_resource.refetch()
                        success_message=async_t_string!(i18n, my_account.email_updated_successfully).to_signal()
                    />

                    <TextField label=async_t_string!(i18n, shared.email).to_signal() name="email" error=error_email />

                    <PasswordField
                        label=async_t_string!(i18n, shared.password).to_signal()
                        name="password"
                        error=error_password
                    />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>
            </section>
        </AuthenticatedPage>
    }
}
