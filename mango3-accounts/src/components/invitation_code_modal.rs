use leptos::prelude::*;

use mango3_leptos_utils::components::{ActionFormError, Modal, SubmitButton, TextField};
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::enums::ActionFormStatus;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::InformationCircleOutlined;

use crate::server_functions::GetInvitationCodeId;

#[component]
pub fn InvitationCodeModal(value: RwSignal<Option<String>>) -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<GetInvitationCodeId>::new();
    let action_value = server_action.value();
    let status = RwSignal::new(ActionFormStatus::Pending);
    let is_open = RwSignal::new(!basic_config.enable_register);
    let support_email_address = basic_config.support_email_address;

    Effect::new(move || match action_value.get() {
        Some(Ok(Some(id))) => {
            value.set(Some(id));
            status.set(ActionFormStatus::Success);
            is_open.set(false);
        }
        Some(Ok(None)) => status.set(ActionFormStatus::Error),
        _ => is_open.set(!basic_config.enable_register),
    });

    view! {
        <Modal is_open=is_open is_closable=false>
            <h4 class="text-lg font-bold">{t!(i18n, accounts.invitation_code)}</h4>

            <div role="alert" class="alert mt-4">
                <InformationCircleOutlined class="self-start my-2" />

                <div>
                    {t!(
                        i18n, accounts.you_can_request_an_invitation_code_by_contacting_us_at_the_following_email_address
                    )} ": " <a class="link link-info" href=format!("mailto:{}", support_email_address) target="_blank">
                        {support_email_address.clone()}
                    </a> "."
                </div>
            </div>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormError message=move || t!(i18n, accounts.failed_to_get_invitation) status=status />

                <TextField label=move || t!(i18n, shared.code) name="code" />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Modal>
    }
}
