use leptos::either::Either;
use leptos::prelude::*;

use leptos_router::hooks::use_navigate;
use mango3_web_utils::async_t_string;
use mango3_web_utils::components::forms::{
    CountryField, FormErrorAlert, FormSuccessModal, PasswordField, SubmitButton, TextField,
};
use mango3_web_utils::context::use_basic_config;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::icons::InformationCircleOutlined;
use mango3_web_utils::pages::GuestPage;
use mango3_web_utils::utils::ToSignalTrait;

use crate::components::InvitationCodeModal;
use crate::server_functions::AttemptToRegister;

#[component]
fn AgreementAlert() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    // let privacy_policy_url = basic_config.privacy_policy_url.clone();
    // let terms_of_service_url = basic_config.terms_of_service_url.clone();

    if basic_config.privacy_policy_url.is_some() || basic_config.terms_of_service_url.is_some() {
        Either::Left(view! {
            <div role="alert" class="alert mt-2 mb-5">
                <InformationCircleOutlined class="self-start my-2" />

                <div>
                    <div class="font-bold">
                        {t!(i18n, accounts.by_submitting_this_form_you_agree_to_the_following)} ": "
                    </div>

                    {basic_config
                        .privacy_policy_url
                        .map(|url| {
                            view! {
                                <div class="text-sm mt-1">
                                    <a class="link link-info" href=url.to_string() target="_blank">
                                        {t!(i18n, shared.privacy_policy)}
                                    </a>
                                </div>
                            }
                        })}

                    {basic_config
                        .terms_of_service_url
                        .map(|url| {
                            view! {
                                <div class="text-sm mt-1">
                                    <a class="link link-info" href=url.to_string() target="_blank">
                                        {t!(i18n, shared.terms_of_service)}
                                    </a>
                                </div>
                            }
                        })}
                </div>
            </div>
        })
    } else {
        Either::Right(())
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let navigate = use_navigate();
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToRegister>::new();
    let action_value = server_action.value();
    let value_invitation_code_id = RwSignal::new(None);
    let text_title = async_t_string!(i18n, shared.register).to_signal();

    view! {
        <GuestPage title=text_title>
            <h1 class="h1">{move || text_title.get()}</h1>

            <InvitationCodeModal value=value_invitation_code_id />

            <ActionForm action=server_action attr:autocomplete="off" attr:novalidate="true" attr:class="form">
                <FormErrorAlert action_value=action_value message=move || t!(i18n, accounts.failed_to_create_user) />

                <Show when=move || !basic_config.enable_register>
                    <input
                        type="hidden"
                        name="invitation_code_id"
                        value=move || value_invitation_code_id.get().map(|id| id.to_string())
                    />
                </Show>

                <TextField
                    action_value=action_value
                    id="username"
                    label=move || t!(i18n, accounts.username)
                    name="username"
                />

                <TextField
                    action_value=action_value
                    id="email"
                    label=move || t!(i18n, shared.email)
                    name="email"
                    input_type="email"
                />

                <PasswordField
                    action_value=action_value
                    id="password"
                    label=move || t!(i18n, shared.password)
                    name="password"
                />

                <TextField
                    action_value=action_value
                    id="full_name"
                    label=move || t!(i18n, shared.full_name)
                    name="full_name"
                />

                <TextField
                    action_value=action_value
                    id="birthdate"
                    input_type="date"
                    label=move || t!(i18n, shared.birthdate)
                    name="birthdate"
                />

                <CountryField
                    action_value=action_value
                    id="country_alpha2"
                    label=move || t!(i18n, shared.country)
                    name="country_alpha2"
                />

                <AgreementAlert />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <FormSuccessModal
                action_value=action_value
                message=move || t!(i18n, accounts.user_created_successfully)
                on_close=move || {
                    navigate(basic_config.home_url.as_ref(), Default::default());
                }
            />

            <div class="max-w-[640px] ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/login">
                    {t!(i18n, accounts.back_to_login)}
                </a>
            </div>
        </GuestPage>
    }
}
