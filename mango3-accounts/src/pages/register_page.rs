use leptos::prelude::*;
use leptos_fluent::tr;

use mango3_leptos_utils::components::*;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::models::ActionFormResp;
use mango3_leptos_utils::pages::Page;

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
    use mango3_leptos_utils::ssr::{expect_core_context, extract_i18n};

    let i18n = extract_i18n().await?;
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

    ActionFormResp::new(&i18n, result).await
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let basic_config = use_basic_config();
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

    view! {
        <Page title=move || tr!("register")>
            <h2 class="text-xl font-bold bottom-4">{move || tr!("register")}</h2>

            <ActionForm
                action=server_action
                attr:autocomplete="off"
                attr:novalidate="true"
                attr:class="max-w-[640px] m-auto"
            >
                <ActionFormAlert
                    action_value=action_value
                    error_message=move || tr!("failed-to-create-user")
                    redirect_to=basic_config.home_url.clone()
                    success_message=move || tr!("user-created-successfully")
                />

                <TextField label=move || tr!("username") name="username" error=error_username />

                <TextField
                    label=move || tr!("email")
                    name="email"
                    input_type="email"
                    error=error_email
                />

                <PasswordField label=move || tr!("password") name="password" error=error_password />

                <TextField label=move || tr!("full-name") name="full_name" error=error_full_name />

                <TextField
                    input_type="date"
                    label=move || tr!("birthdate")
                    name="birthdate"
                    error=error_birthdate
                />

                <CountryField
                    label=move || tr!("country")
                    name="country_alpha2"
                    error=error_country_alpha2
                />

                <SubmitButton is_loading=server_action.pending() />
            </ActionForm>
        </Page>
    }
}
