use leptos::either::Either;
use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::{ActionFormError, SubmitButton, TextField};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::InformationCircleOutlined;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::server_functions::GetInvitationCodeId;

#[component]
pub fn InvitationCodeDialog(value: RwSignal<Option<String>>) -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<GetInvitationCodeId>::new();
    let action_value = server_action.value();
    let support_email_address = basic_config.support_email_address;

    Effect::new(move || {
        if let Some(Ok(Some(id))) = action_value.get() {
            value.set(Some(id));
        }
    });

    view! {
        <Show when=move || !basic_config.enable_register && value.get().is_none()>
            <div class="modal modal-open">
                <div class="modal-box">
                    <h4 class="text-lg font-bold">{t!(i18n, accounts.invitation_code)}</h4>

                    <div role="alert" class="alert mt-4">
                        <InformationCircleOutlined class="self-start my-2" />

                        <div>
                            {t!(
                                i18n, accounts.you_can_request_an_invitation_code_by_contacting_us_at_the_following_email_address
                            )} ": "
                            <a class="link link-info" href=format!("mailto:{}", support_email_address) target="_blank">
                                {support_email_address.clone()}
                            </a> "."
                        </div>
                    </div>

                    <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                        {move || {
                            if let Some(Ok(None)) = action_value.get() {
                                Either::Left(
                                    view! {
                                        <ActionFormError message=async_t_string!(
                                            i18n, accounts.failed_to_get_invitation
                                        )
                                            .to_signal() />
                                    },
                                )
                            } else {
                                Either::Right(())
                            }
                        }}

                        <TextField label=async_t_string!(i18n, shared.code).to_signal() name="code" />

                        <SubmitButton is_loading=server_action.pending() />
                    </ActionForm>
                </div>
            </div>
        </Show>
    }
}
