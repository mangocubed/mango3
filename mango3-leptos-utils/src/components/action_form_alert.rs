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
    #[prop(into, optional)] error_message: ViewFn,
    #[prop(into, optional)] on_success: Option<BoxedFn>,
    #[prop(into, optional)] redirect_to: Option<TextProp>,
    #[prop(into)] success_message: ViewFn,
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

        if let Some(to) = &redirect_to {
            navigate(&to.get(), Default::default())
        }
    });

    move || match ActionFormResp::from(action_value).success {
        Some(true) => EitherOf3::A(view! {
            <dialog class="modal" class:modal-open=move || !is_done.get()>
                <div class="modal-box">
                    <div>{success_message.run()}</div>
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
        Some(false) => {
            is_done.set(false);
            EitherOf3::B(view! { <ActionFormError message=error_message.clone() /> })
        }
        _ => {
            is_done.set(false);
            EitherOf3::C(())
        }
    }
}

#[component]
pub fn ActionFormError(#[prop(into)] message: ViewFn) -> impl IntoView {
    view! {
        <div class="pt-2 pb-2 has-[div:empty]:hidden">
            <div role="alert" class="alert alert-error">
                {message.run()}
            </div>
        </div>
    }
}
