use leptos::prelude::*;

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::*;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::icons::InformationCircleOutlined;
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;
use mango3_leptos_utils::utils::ToSignalTrait;

use crate::components::InvitationCodeDialog;
use crate::server_functions::AttemptToRegister;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToRegister>::new();
    let action_value = server_action.value();
    let error_username = RwSignal::new(None);
    let error_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);
    let error_full_name = RwSignal::new(None);
    let error_birthdate = RwSignal::new(None);
    let error_country_alpha2 = RwSignal::new(None);
    let value_invitation_code_id = RwSignal::new(None);
    let text_title = async_t_string!(i18n, shared.register).to_signal();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_username.set(response.error("username"));
        error_email.set(response.error("email"));
        error_password.set(response.error("password"));
        error_full_name.set(response.error("full-name"));
        error_birthdate.set(response.error("birthdate"));
        error_country_alpha2.set(response.error("country-alpha2"));
    });

    let privacy_policy_url = basic_config.privacy_policy_url.clone();
    let terms_of_service_url = basic_config.terms_of_service_url.clone();
    let has_privacy_policy = !privacy_policy_url.is_empty();
    let has_terms_of_service = !terms_of_service_url.is_empty();

    view! {
        <GuestPage title=text_title>
            <h2 class="h2">{move || text_title.get()}</h2>

            <InvitationCodeDialog value=value_invitation_code_id />

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t!(i18n, accounts.failed_to_create_user)
                    redirect_to=basic_config.home_url.clone()
                    success_message=move || t!(i18n, accounts.user_created_successfully)
                />

                <Show when=move || !basic_config.enable_register>
                    <input type="hidden" name="invitation_code_id" value=value_invitation_code_id />
                </Show>

                <TextField label=move || t!(i18n, accounts.username) name="username" error=error_username />

                <TextField label=move || t!(i18n, shared.email) name="email" input_type="email" error=error_email />

                <PasswordField label=move || t!(i18n, shared.password) name="password" error=error_password />

                <TextField label=move || t!(i18n, shared.full_name) name="full_name" error=error_full_name />

                <TextField
                    input_type="date"
                    label=move || t!(i18n, shared.birthdate)
                    name="birthdate"
                    error=error_birthdate
                />

                <CountryField label=move || t!(i18n, shared.country) name="country_alpha2" error=error_country_alpha2 />

                <Show when=move || {
                    has_privacy_policy || has_terms_of_service
                }>
                    {
                        let privacy_policy_url = privacy_policy_url.clone();
                        let terms_of_service_url = terms_of_service_url.clone();
                        move || {
                            let privacy_policy_url = privacy_policy_url.clone();
                            let terms_of_service_url = terms_of_service_url.clone();
                            view! {
                                <div role="alert" class="alert mt-2 mb-5">
                                    <InformationCircleOutlined class="self-start my-2" />
                                    <div>
                                        <div class="font-bold">
                                            {t!(i18n, accounts.by_submitting_this_form_you_agree_to_the_following)} ": "
                                        </div>
                                        <Show when=move || { has_privacy_policy }>
                                            <div class="text-sm mt-1">
                                                <a
                                                    class="link link-info"
                                                    href=privacy_policy_url.clone()
                                                    target="_blank"
                                                >
                                                    {t!(i18n, shared.privacy_policy)}
                                                </a>
                                            </div>
                                        </Show>
                                        <Show when=move || has_terms_of_service>
                                            <div class="text-sm mt-1">
                                                <a
                                                    class="link link-info"
                                                    href=terms_of_service_url.clone()
                                                    target="_blank"
                                                >
                                                    {t!(i18n, shared.terms_of_service)}
                                                </a>
                                            </div>
                                        </Show>
                                    </div>
                                </div>
                            }
                        }
                    }
                </Show>

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <div class="max-w-[640px] ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/login">
                    {t!(i18n, accounts.back_to_login)}
                </a>
            </div>
        </GuestPage>
    }
}
