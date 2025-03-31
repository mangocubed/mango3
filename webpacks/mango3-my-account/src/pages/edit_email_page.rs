use leptos::prelude::*;
use web_sys::HtmlFormElement;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal, PasswordField, SubmitButton, TextField};
use mango3_web_utils::components::CurrentUser;
use mango3_web_utils::context::use_current_user_resource;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::models::FormResp;
use mango3_web_utils::pages::AuthenticatedPage;
use mango3_web_utils::utils::ToSignalTrait;

use crate::components::EmailConfirmationBadge;
use crate::server_functions::AttemptToUpdateEmail;

#[component]
pub fn EditEmailPage() -> impl IntoView {
    let i18n = use_i18n();
    let current_user_resource = use_current_user_resource();
    let form_node_ref = NodeRef::new();
    let server_action = ServerAction::<AttemptToUpdateEmail>::new();
    let action_value = server_action.value();
    let text_title = async_t_string!(i18n, my_account.edit_email).to_signal();

    Effect::new(move || {
        let response = FormResp::from(action_value);

        if response.is_success() {
            form_node_ref.with(|form| form.as_ref().map(|f: &HtmlFormElement| f.reset()));
        }
    });

    view! {
        <AuthenticatedPage title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <section class="max-w-[640px] w-full mx-auto">
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

            <section class="max-w-[640px] w-full mx-auto mt-4">
                <h3 class="h3">{t!(i18n, my_account.change_email)}</h3>

                <ActionForm
                    node_ref=form_node_ref
                    action=server_action
                    attr:autocomplete="off"
                    attr:novalidate="true"
                    attr:class="form"
                >
                    <FormErrorAlert
                        action_value=action_value
                        message=move || t!(i18n, my_account.failed_to_update_email)
                    />

                    <TextField action_value=action_value id="email" label=move || t!(i18n, shared.email) name="email" />

                    <PasswordField
                        action_value=action_value
                        id="password"
                        label=move || t!(i18n, shared.password)
                        name="password"
                    />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>

                <FormSuccessModal
                    action_value=action_value
                    message=move || t!(i18n, my_account.email_updated_successfully)
                    on_close=move || current_user_resource.refetch()
                />
            </section>
        </AuthenticatedPage>
    }
}
