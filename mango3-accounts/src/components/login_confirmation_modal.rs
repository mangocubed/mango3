use leptos::prelude::*;

use mango3_leptos_utils::components::forms::{FormErrorAlert, SubmitButton, TextField};
use mango3_leptos_utils::components::Modal;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::InformationCircleOutlined;
use mango3_leptos_utils::models::FormResp;

use crate::server_functions::AttemptToConfirmLogin;

#[component]
pub fn LoginConfirmationModal(is_open: RwSignal<bool>, #[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToConfirmLogin>::new();
    let action_value = server_action.value();
    let error_alert_is_active = RwSignal::new(false);

    Effect::new(move || {
        let response = FormResp::from(action_value);

        if response.is_success() {
            is_open.set(false);
            on_success.run(());
        }

        error_alert_is_active.set(response.is_invalid());
    });

    view! {
        <Modal is_open=is_open>
            <h4 class="text-lg font-bold">{t!(i18n, accounts.confirm_login)}</h4>

            <div role="alert" class="alert mt-4">
                <InformationCircleOutlined class="self-start my-2" />

                <div>{t!(i18n, accounts.a_confirmation_code_has_been_sent_to_your_email_address)}"."</div>
            </div>

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert action_value=action_value message=move || t!(i18n, accounts.failed_to_confirm_login) />

                <TextField action_value=action_value id="code" label=move || t!(i18n, shared.code) name="code" />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Modal>
    }
}
