use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_location, use_navigate};

use mango3_leptos_utils::async_t_string;
use mango3_leptos_utils::components::ConfirmationModal;
use mango3_leptos_utils::context::use_basic_config;
use mango3_leptos_utils::i18n::{t, use_i18n};

use crate::server_functions::AttemptToLogout;

#[component]
pub fn IndexParentPage() -> impl IntoView {
    let basic_config = use_basic_config();
    let location = use_location();
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

    let menu_items = move || {
        [
            ("/", async_t_string!(i18n, shared.home)),
            ("/edit-profile", async_t_string!(i18n, my_account.edit_profile)),
            ("/edit-email", async_t_string!(i18n, my_account.edit_email)),
            ("/change-password", async_t_string!(i18n, shared.change_password)),
        ]
    };

    view! {
        <div class="flex grow gap-4">
            <ul class="menu bg-base-200 rounded-box w-56">
                <For each=menu_items key=|(href, _)| href.to_owned() let:data>
                    <li>
                        <a class:menu-active=move || location.pathname.get() == data.0 href=data.0>
                            {move || data.1.get()}
                        </a>
                    </li>
                </For>

                <li>
                    <a on:click=move |_| { show_logout_confirmation.set(true) }>{t!(i18n, my_account.logout)}</a>
                </li>
            </ul>

            <div class="grow ml-4">
                <Outlet />
            </div>
        </div>

        <ConfirmationModal
            is_open=show_logout_confirmation
            on_accept=move || {
                logout_server_action.dispatch(AttemptToLogout {});
            }
        >
            {t!(i18n, my_account.are_you_sure_you_want_to_logout)}
        </ConfirmationModal>
    }
}
