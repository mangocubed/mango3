use leptos::either::EitherOf3;
use leptos::prelude::*;

use mango3_web_utils::async_t_string;
use mango3_web_utils::components::{
    ConfirmationModal, InfiniteScroll, InfiniteScrollControllerTrait, InfiniteScrollLocalResourceController, UserCard,
    UserTag,
};
use mango3_web_utils::context::use_basic_config;
use mango3_web_utils::i18n::{t, use_i18n};
use mango3_web_utils::presenters::{MutPresenter, UserMinPresenter};
use mango3_web_utils::utils::ToSignalTrait;

use crate::components::AdminPageContainer;
use crate::server_functions::{get_users, AttemptToDisableUser, AttemptToEnableUser};

#[component]
pub fn UsersPage() -> impl IntoView {
    let i18n = use_i18n();
    let basic_config = use_basic_config();
    let controller = InfiniteScrollLocalResourceController::new(|after| {
        LocalResource::new(move || async move { get_users(after.get()).await })
    });
    let text_title = async_t_string!(i18n, admin.users).to_signal();
    let server_action_disable_user = ServerAction::<AttemptToDisableUser>::new();
    let action_value_disable_user = server_action_disable_user.value();
    let server_action_enable_user = ServerAction::<AttemptToEnableUser>::new();
    let action_value_enable_user = server_action_enable_user.value();
    let disable_user = RwSignal::<Option<UserMinPresenter>>::new(None);
    let enable_user = RwSignal::<Option<UserMinPresenter>>::new(None);
    let show_disable_confirmation = RwSignal::new(false);
    let show_enable_confirmation = RwSignal::new(false);

    Effect::new({
        let controller = controller.clone();
        move || {
            let response = MutPresenter::from(action_value_disable_user);

            if let Some(true) = response.success {
                controller.clear_and_refetch();
                enable_user.set(None);
            }
        }
    });

    Effect::new({
        let controller = controller.clone();
        move || {
            let response = MutPresenter::from(action_value_enable_user);

            if let Some(true) = response.success {
                controller.clear_and_refetch();
                enable_user.set(None);
            }
        }
    });

    view! {
        <AdminPageContainer title=text_title>
            <h1 class="h1">{move || text_title.get()}</h1>

            <section class="max-w-[720px] w-full mx-auto">
                <ConfirmationModal
                    is_open=show_disable_confirmation
                    on_accept=move || {
                        let user_id = disable_user.get().unwrap().id;
                        server_action_disable_user
                            .dispatch(AttemptToDisableUser {
                                id: user_id,
                            });
                    }
                >
                    <div>{t!(i18n, admin.are_you_sure_you_want_to_disable_this_user)}</div>

                    {move || disable_user.get().map(|user| view! { <UserTag class="justify-center my-3" user=user /> })}
                </ConfirmationModal>

                <ConfirmationModal
                    is_open=show_enable_confirmation
                    on_accept=move || {
                        let user_id = enable_user.get().unwrap().id;
                        server_action_enable_user.dispatch(AttemptToEnableUser { id: user_id });
                    }
                >
                    <div>{t!(i18n, admin.are_you_sure_you_want_to_enable_this_user)}</div>

                    {move || enable_user.get().map(|user| view! { <UserTag class="justify-center my-3" user=user /> })}
                </ConfirmationModal>

                <InfiniteScroll controller=controller key=|user: &UserMinPresenter| user.id let:user>
                    <UserCard
                        user=user.clone()
                        hashtags_base_url=basic_config.home_url.to_string()
                        actions=move || {
                            let user = user.clone();
                            if user.is_disabled {
                                EitherOf3::A(
                                    view! {
                                        <button
                                            class="btn btn-ghost font-bold"
                                            on:click=move |_| {
                                                enable_user.set(Some(user.clone()));
                                                show_enable_confirmation.set(true);
                                            }
                                        >
                                            {t!(i18n, admin.enable)}
                                        </button>
                                    },
                                )
                            } else if user.role == "user" {
                                EitherOf3::B(
                                    view! {
                                        <button
                                            class="btn btn-ghost font-bold"
                                            on:click=move |_| {
                                                disable_user.set(Some(user.clone()));
                                                show_disable_confirmation.set(true);
                                            }
                                        >
                                            {t!(i18n, admin.disable)}
                                        </button>
                                    },
                                )
                            } else {
                                EitherOf3::C(())
                            }
                        }
                    />
                </InfiniteScroll>
            </section>
        </AdminPageContainer>
    }
}
