use leptos::prelude::*;

use leptos_i18n::t_string;
use mango3_leptos_utils::components::*;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::GuestPage;

#[server]
pub async fn attempt_to_login(username_or_email: String, password: String) -> Result<ActionFormResp, ServerFnError> {
    use mango3_core::models::User;
    use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n, require_no_authentication, start_user_session};

    let i18n = extract_i18n().await?;

    if !require_no_authentication().await? {
        return ActionFormResp::new_with_error(&i18n);
    }

    let core_context = expect_core_context();

    let result = User::authenticate(&core_context, &username_or_email, &password).await;

    if let Ok(ref user) = result {
        start_user_session(&core_context, &user).await?;
    }

    ActionFormResp::new(&i18n, result)
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let basic_config = use_basic_config();
    let i18n = use_i18n();
    let server_action = ServerAction::<AttemptToLogin>::new();
    let action_value = server_action.value();
    let error_username_or_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_username_or_email.set(response.error("username-or-email"));
        error_password.set(response.error("password"));
    });

    let title = move || t_string!(i18n, shared.login);

    view! {
        <GuestPage title=title>
            <h2 class="text-xl font-bold mb-4">{title}</h2>

            <ActionForm
                action=server_action
                attr:autocomplete="off"
                attr:novalidate="true"
                attr:class="max-w-[640px] m-auto"
            >
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || t_string!(i18n, accounts.failed_to_authenticate_user)
                    redirect_to=basic_config.home_url.clone()
                    success_message=move || {
                        t_string!(i18n, accounts.user_authenticated_successfully)
                    }
                />

                <TextField
                    label=move || t_string!(i18n, accounts.username_or_email)
                    name="username_or_email"
                    error=error_username_or_email
                />

                <PasswordField
                    label=move || t_string!(i18n, accounts.password)
                    name="password"
                    error=error_password
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <div class="max-w-[640px]  ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/register">
                    {t!(i18n, accounts.i_dont_have_an_account)}
                </a>
            </div>
        </GuestPage>
    }
}
