use leptos::prelude::*;
use leptos_i18n::t_string;
use leptos_router::hooks::use_navigate;

use mango3_leptos_utils::components::ConfirmationDialog;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};
use mango3_leptos_utils::pages::AuthenticatedPage;

#[server]
pub async fn attempt_to_logout() -> Result<(), ServerFnError> {
    use mango3_leptos_utils::ssr::{expect_core_context, finish_user_session, require_authentication};

    if !require_authentication().await? {
        return Ok(());
    }

    let core_context = expect_core_context();

    finish_user_session(&core_context).await?;

    Ok(())
}

#[component]
pub fn IndexPage() -> impl IntoView {
    let basic_config = use_basic_config();
    let navigate = use_navigate();
    let i18n = use_i18n();
    let show_logout_confirmation = RwSignal::new(false);
    let logout_server_action = ServerAction::<AttemptToLogout>::new();
    let logout_action_value = logout_server_action.value();

    Effect::new(move || {
        if let Some(()) = logout_action_value.get().and_then(|result| result.ok()) {
            navigate(&basic_config.login_url, Default::default());
        }
    });

    view! {
        <AuthenticatedPage title=move || t_string!(i18n, shared.home)>
            <ul class="menu grow bg-base-200 rounded-box w-56">
                <li>
                    <a on:click=move |_| {
                        show_logout_confirmation.set(true)
                    }>{move || t!(i18n, my_account.logout)}</a>
                </li>
            </ul>

            <ConfirmationDialog
                is_open=show_logout_confirmation
                on_accept=move || {
                    logout_server_action.dispatch(AttemptToLogout {});
                }
            >
                {move || t!(i18n, my_account.are_you_sure_you_want_to_logout)}
            </ConfirmationDialog>
        </AuthenticatedPage>
    }
}
