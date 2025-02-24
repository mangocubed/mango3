use leptos::prelude::*;
use web_sys::HtmlFormElement;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::forms::{ActionFormErrorAlert, ActionFormSuccessModal};
use mango3_leptos_utils::components::{CurrentUser, PasswordField, SubmitButton, TextField};
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
    let form_node_ref = NodeRef::new();
    let server_action = ServerAction::<AttemptToUpdateEmail>::new();
    let action_value = server_action.value();
    let error_alert_is_active = RwSignal::new(false);
    let error_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);
    let success_modal_is_open = RwSignal::new(false);
    let text_title = async_t_string!(i18n, my_account.edit_email).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        if response.is_success() {
            form_node_ref.with(|form| form.as_ref().map(|f: &HtmlFormElement| f.reset()));
            success_modal_is_open.set(true);
        }

        error_alert_is_active.set(response.is_invalid());
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
                        view! {
                            <span>{user.email}</span>

                            <EmailConfirmationBadge is_confirmed=user.email_is_confirmed />
                        }
                    } />
                </div>
            </section>

            <section class="max-w-[640px] w-full ml-auto mr-auto mt-4">
                <h3 class="h3">{t!(i18n, my_account.change_email)}</h3>

                <ActionForm
                    node_ref=form_node_ref
                    action=server_action
                    attr:autocomplete="off"
                    attr:novalidate="true"
                    attr:class="form"
                >
                    <ActionFormErrorAlert
                        is_active=error_alert_is_active
                        message=move || t!(i18n, my_account.failed_to_update_email)
                    />

                    <TextField label=move || t!(i18n, shared.email) name="email" error=error_email />

                    <PasswordField label=move || t!(i18n, shared.password) name="password" error=error_password />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>

                <ActionFormSuccessModal
                    is_open=success_modal_is_open
                    message=move || t!(i18n, my_account.email_updated_successfully)
                    on_close=move || current_user_resource.refetch()
                />
            </section>
        </AuthenticatedPage>
    }
}
