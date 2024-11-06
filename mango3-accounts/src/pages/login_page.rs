use leptos::prelude::*;
use leptos_fluent::tr;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::context::use_basic_config;
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
    let server_action = ServerAction::<AttemptToLogin>::new();
    let action_value = server_action.value();
    let error_username_or_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        error_username_or_email.set(response.error("username-or-email"));
        error_password.set(response.error("password"));
    });

    let title = move || tr!("login");

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
                    error_message=move || tr!("failed-to-authenticate-user")
                    redirect_to=basic_config.home_url.clone()
                    success_message=move || tr!("user-authenticated-successfully")
                />

                <TextField
                    label=move || tr!("username-or-email")
                    name="username_or_email"
                    error=error_username_or_email
                />

                <PasswordField label=move || tr!("password") name="password" error=error_password />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>

            <div class="max-w-[640px]  ml-auto mr-auto mt-4">
                <a class="btn btn-block btn-outline" href="/register">
                    {move || tr!("i-dont-have-an-account")}
                </a>
            </div>
        </GuestPage>
    }
}
