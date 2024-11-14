use leptos::prelude::*;
use leptos_i18n::t_string;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;

#[server]
pub async fn attempt_to_register(
    username: String,
    email: String,
    password: String,
    full_name: String,
    birthdate: String,
    country_alpha2: String,
) -> Result<ActionFormResp, ServerFnError> {
    use mango3_core::models::User;
    use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, require_no_authentication, start_user_session};

    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let result = User::insert(
        &core_context,
        &username,
        &email,
        &password,
        &full_name,
        &birthdate,
        i18n.0.language.as_str(),
        &country_alpha2,
    )
    .await;

    if let Ok(ref user) = result {
        start_user_session(&core_context, &user).await?;
    }

    ActionFormResp::new(&i18n, result)
}

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

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_username.set(response.error("username"));
        error_email.set(response.error("email"));
        error_password.set(response.error("password"));
        error_full_name.set(response.error("full-name"));
        error_birthdate.set(response.error("birthdate"));
        error_country_alpha2.set(response.error("country-alpha2"));
    });

    let title = move || t_string!(i18n, shared.register);

    view! {
        <GuestPage title=title>
            <h2 class="h2">{title}</h2>

            <ActionForm
                action=server_action
                attr:autocomplete="off"
                attr:novalidate="true"
                attr:class="form"
            >
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t_string!(i18n, accounts.failed_to_create_user)
                    redirect_to=basic_config.home_url.clone()
                    success_message=move || t_string!(i18n, accounts.user_created_successfully)
                />

                <TextField
                    label=move || t_string!(i18n, accounts.username)
                    name="username"
                    error=error_username
                />

                <TextField
                    label=move || t_string!(i18n, shared.email)
                    name="email"
                    input_type="email"
                    error=error_email
                />

                <PasswordField
                    label=move || t_string!(i18n, shared.password)
                    name="password"
                    error=error_password
                />

                <TextField
                    label=move || t_string!(i18n, shared.full_name)
                    name="full_name"
                    error=error_full_name
                />

                <TextField
                    input_type="date"
                    label=move || t_string!(i18n, shared.birthdate)
                    name="birthdate"
                    error=error_birthdate
                />

                <CountryField
                    label=move || t_string!(i18n, shared.country)
                    name="country_alpha2"
                    error=error_country_alpha2
                />

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
