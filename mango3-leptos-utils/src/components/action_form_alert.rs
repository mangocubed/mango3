use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_router::hooks::use_navigate;
use server_fn::error::NoCustomError;

use crate::models::ActionFormResp;

use super::BoxedFn;

#[component]
pub fn ActionFormAlert(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(into, optional)] error_message: Option<TextProp>,
    #[prop(into, optional)] on_success: Option<BoxedFn>,
    #[prop(into, optional)] redirect_to: Option<String>,
    #[prop(into)] success_message: TextProp,
) -> impl IntoView {
    let navigate = use_navigate();
    let is_done = RwSignal::new(false);

    Effect::new(move || {
        if !is_done.get() {
            return;
        }

        if let Some(on_success) = on_success.as_ref() {
            on_success.0();
        }

        if let Some(to) = redirect_to.clone() {
            navigate(&to, Default::default())
        }
    });

    let success_msg = move || success_message.clone().get();

    move || match ActionFormResp::from(action_value).success {
        Some(true) => EitherOf3::A(view! {
            <dialog class="modal" class:modal-open=move || !is_done.get()>
                <div class="modal-box">
                    <div>{success_msg.clone()}</div>
                    <div class="modal-action">
                        <button
                            class="btn btn-primary"
                            on:click=move |event| {
                                event.prevent_default();
                                is_done.set(true);
                            }
                        >
                            "Ok"
                        </button>
                    </div>
                </div>
            </dialog>
        }),
        Some(false) => EitherOf3::B(
            error_message
                .as_ref()
                .map(|error_msg| view! { <ActionFormError message=error_msg.clone() /> }),
        ),
        _ => EitherOf3::C(()),
    }
}

#[component]
pub fn ActionFormError(#[prop(into)] message: TextProp) -> impl IntoView {
    view! {
        <div class="pt-2 pb-2">
            <div role="alert" class="alert alert-error">
                {move || message.get()}
            </div>
        </div>
    }
}
