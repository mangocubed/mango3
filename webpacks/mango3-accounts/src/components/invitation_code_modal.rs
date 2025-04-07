use leptos::prelude::*;
use uuid::Uuid;

use mango3_web_utils::components::forms::{FormErrorAlert, SubmitButton, TextField};
use mango3_web_utils::components::Modal;
use mango3_web_utils::context::use_basic_config;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::InformationCircleOutlined;
use mango3_web_utils::presenters::MutPresenter;

use crate::server_functions::AttemptToGetInvitationCodeId;

#[component]
pub fn InvitationCodeModal(value: RwSignal<Option<Uuid>>) -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToGetInvitationCodeId>::new();
    let action_value = server_action.value();
    let is_open = RwSignal::new(!basic_config.enable_register);
    let support_email_address = basic_config.support_email_address;

    Effect::new(move || {
        let response = MutPresenter::from(action_value);

        if response.is_success() {
            value.set(response.data);
            is_open.set(false);
        }
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
                <FormErrorAlert action_value=action_value message=move || t!(i18n, accounts.failed_to_get_invitation) />

                <TextField action_value=action_value id="code" label=move || t!(i18n, shared.code) name="code" />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Modal>
    }
}
