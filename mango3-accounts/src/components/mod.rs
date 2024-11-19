use leptos::either::Either;
use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, ActionFormError, PasswordField, SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;

use crate::server_functions::AttemptToUpdatePasswordWithCode;

mod invitation_code_dialog;

pub use invitation_code_dialog::InvitationCodeDialog;

#[component]
pub fn ResetPasswordDialog(
    #[prop(into)] username_or_email: Signal<String>,
    #[prop(into)] is_open: RwSignal<bool>,
) -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToUpdatePasswordWithCode>::new();
    let action_value = server_action.value();
    let error_code = RwSignal::new(None);
    let error_new_password = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        if let Some(true) = response.success {
            is_open.set(false);
        }

        error_code.set(response.error("code"));
        error_new_password.set(response.error("new-password"));
    });

    view! {
        <ActionFormAlert
            action_value=action_value
            redirect_to="/login"
            success_message=move || { t_string!(i18n, shared.password_updated_successfully) }
        />

        <div class="modal" class:modal-open=is_open>
            <div class="modal-box">
                <button
                    class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                    on:click=move |_| is_open.set(false)
                >
                    "✕"
                </button>

                <h4 class="text-lg font-bold">{t!(i18n, shared.change_password)}</h4>

                <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                    {move || {
                        if let Some(false) = ActionFormResp::from(action_value).success {
                            Either::Left(
                                view! {
                                    <ActionFormError message=move || {
                                        t_string!(i18n, shared.failed_to_update_password)
                                    } />
                                },
                            )
                        } else {
                            Either::Right(())
                        }
                    }}

                    <input type="hidden" name="username_or_email" value=username_or_email />

                    <TextField label=move || t_string!(i18n, shared.code) name="code" error=error_code />

                    <PasswordField
                        label=move || t_string!(i18n, shared.new_password)
                        name="new_password"
                        error=error_new_password
                    />

                    <SubmitButton is_loading=server_action.pending() />
                </ActionForm>
            </div>
        </div>
    }
}
