use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_router::hooks::use_navigate;
use server_fn::error::NoCustomError;

use crate::enums::ActionFormStatus;
use crate::models::ActionFormResp;

use super::{BoxedFn, Modal};

#[component]
pub fn ActionFormAlert(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(into, optional)] error_message: ViewFn,
    #[prop(into, optional)] on_success: Option<BoxedFn>,
    #[prop(into, optional)] redirect_to: Option<TextProp>,
    #[prop(default = RwSignal::new(ActionFormStatus::Pending), into)] status: RwSignal<ActionFormStatus>,
    #[prop(into)] success_message: ViewFn,
) -> impl IntoView {
    let navigate = use_navigate();

    Effect::new(move || {
        let response = ActionFormResp::from(action_value);

        match response.success {
            Some(true) => {
                status.set(ActionFormStatus::Success);
            }
            Some(false) => {
                status.set(ActionFormStatus::Error);
            }
            _ => {
                status.set(ActionFormStatus::Pending);
            }
        }
    });

    Effect::new(move || {
        if !status.get().is_done() {
            return;
        }

        if let Some(on_success) = on_success.as_ref() {
            on_success.0();
        }

        if let Some(to) = &redirect_to {
            navigate(&to.get(), Default::default())
        }
    });

    view! {
        <ActionFormError message=error_message status=status />

        <SuccessModal message=success_message status=status />
    }
}

#[component]
pub fn ActionFormError(#[prop(into)] message: ViewFn, status: RwSignal<ActionFormStatus>) -> impl IntoView {
    view! {
        <div class="pt-2 pb-2 has-[div:empty]:hidden" class:hidden=move || !status.get().is_error()>
            <div role="alert" class="alert alert-error">
                {message.run()}
            </div>
        </div>
    }
}

#[component]
pub fn SuccessModal(#[prop(into)] message: ViewFn, status: RwSignal<ActionFormStatus>) -> impl IntoView {
    let is_open = RwSignal::new(false);

    Effect::new(move || {
        is_open.set(status.get().is_success());
    });

    view! {
        <Modal is_closable=false is_open=is_open>
            <div>{message.run()}</div>
            <div class="modal-action">
                <button
                    class="btn btn-primary"
                    on:click=move |event| {
                        event.prevent_default();
                        status.set(ActionFormStatus::Done)
                    }
                >
                    "Ok"
                </button>
            </div>
        </Modal>
    }
}
