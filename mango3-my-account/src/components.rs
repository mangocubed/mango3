use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::{ActionFormAlert, ActionFormError, SubmitButton, TextField};
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::CheckMini;
use mango3_leptos_utils::models::ActionFormResp;

use crate::server_functions::{AttemptToConfirmEmail, AttemptToSendEmailConfirmationCode};

#[component]
pub fn EmailConfirmationBadge(#[prop(into)] is_confirmed: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();
    let server_action_send_code = ServerAction::<AttemptToSendEmailConfirmationCode>::new();
    let action_value_send_code = server_action_send_code.value();
    let server_action_confirm = ServerAction::<AttemptToConfirmEmail>::new();
    let action_value_confirm = server_action_confirm.value();
    let show_dialog = RwSignal::new(false);
    let error_code = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value_send_code);

        if let Some(true) = response.success {
            show_dialog.set(true)
        }

        error_code.set(response.error("code"));
    });

    Effect::new(move || {
        let response = ActionFormResp::from(action_value_confirm);

        if let Some(true) = response.success {
            show_dialog.set(false);
        }

        error_code.set(response.error("code"));
    });

    let on_click_send_code = move |event: MouseEvent| {
        event.prevent_default();
        server_action_send_code.dispatch(AttemptToSendEmailConfirmationCode {});
    };

    if is_confirmed.get() {
        Either::Left(
            view! { <div class="badge  badge-outline badge-lg">{t!(i18n, my_account.confirmed)} <CheckMini /></div> },
        )
    } else {
        Either::Right(view! {
            <ActionFormAlert
                action_value=action_value_confirm
                redirect_to="/"
                success_message=move || { t_string!(i18n, my_account.email_confirmed_successfully) }
            />

            <div class="modal" class:modal-open=show_dialog>
                <div class="modal-box">
                    <button
                        class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                        on:click=move |_| show_dialog.set(false)
                    >
                        "✕"
                    </button>

                    <h4 class="text-lg font-bold">{t!(i18n, my_account.confirm_email)}</h4>

                    <ActionForm
                        action=server_action_confirm
                        attr:autocomplete="off"
                        attr:novalidate="true"
                        attr:class="form"
                    >
                        {move || {
                            if let Some(false) = ActionFormResp::from(action_value_confirm).success {
                                Either::Left(
                                    view! {
                                        <ActionFormError message=move || {
                                            t_string!(i18n, my_account.failed_to_confirm_email)
                                        } />
                                    },
                                )
                            } else {
                                Either::Right(())
                            }
                        }}

                        <TextField label=move || t_string!(i18n, shared.code) name="code" error=error_code />

                        <SubmitButton is_loading=server_action_confirm.pending() />
                    </ActionForm>
                </div>
            </div>

            <button class="btn btn-outline" on:click=on_click_send_code>
                {move || {
                    if server_action_send_code.pending().get() {
                        Either::Left(view! { <span class="loading loading-spinner" /> })
                    } else {
                        Either::Right(t!(i18n,  my_account.send_confirmation_code))
                    }
                }}
            </button>
        })
    }
}
