use leptos::either::Either;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

use mango3_web_utils::components::forms::{FormErrorAlert, FormSuccessModal, SubmitButton, TextField};
use mango3_web_utils::components::Modal;
use mango3_web_utils::context::use_current_user_resource;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::CheckMini;
use mango3_web_utils::models::FormResp;

use crate::server_functions::{AttemptToConfirmEmail, AttemptToSendEmailConfirmationCode};

#[component]
pub fn EmailConfirmationModal(is_open: RwSignal<bool>) -> impl IntoView {
    let i18n = use_i18n();
    let current_user_resource = use_current_user_resource();
    let server_action = ServerAction::<AttemptToConfirmEmail>::new();
    let action_value = server_action.value();
    let success_modal_is_open = RwSignal::new(false);

    Effect::new(move || {
        let response = FormResp::from(action_value);

        if let Some(true) = response.success {
            is_open.set(false);
            success_modal_is_open.set(true);
        }
    });

    view! {
        <Modal is_open=is_open>
            <h4 class="text-lg font-bold">{t!(i18n, my_account.confirm_email)}</h4>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert
                    action_value=action_value
                    message=move || t!(i18n, my_account.failed_to_confirm_email)
                />

                <TextField action_value=action_value id="code" label=move || t!(i18n, shared.code) name="code" />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Modal>

        <FormSuccessModal
            is_open=success_modal_is_open
            message=move || t!(i18n, my_account.email_confirmed_successfully)
            on_close=move || current_user_resource.refetch()
        />
    }
}

#[component]
pub fn EmailConfirmationBadge(#[prop(into)] is_confirmed: Signal<bool>) -> impl IntoView {
    let i18n = use_i18n();
    let send_code_server_action = ServerAction::<AttemptToSendEmailConfirmationCode>::new();
    let send_code_action_value = send_code_server_action.value();
    let confirmation_modal_is_open = RwSignal::new(false);

    Effect::new(move || {
        let response = FormResp::from(send_code_action_value);

        if let Some(true) = response.success {
            confirmation_modal_is_open.set(true)
        }
    });

    let on_click_send_code = move |event: MouseEvent| {
        event.prevent_default();
        send_code_server_action.dispatch(AttemptToSendEmailConfirmationCode {});
    };

    move || {
        if is_confirmed.get() {
            Either::Left(
                view! { <div class="badge badge-outline badge-accept">{t!(i18n, my_account.confirmed)} <CheckMini /></div> },
            )
        } else {
            Either::Right(view! {
                <button class="btn btn-outline" on:click=on_click_send_code>
                    {move || {
                        if send_code_server_action.pending().get() {
                            Either::Left(view! { <span class="loading loading-spinner" /> })
                        } else {
                            Either::Right(t!(i18n,  my_account.send_confirmation_code))
                        }
                    }}
                </button>

                <EmailConfirmationModal is_open=confirmation_modal_is_open />
            })
        }
    }
}
