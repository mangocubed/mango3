use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use leptos_router::hooks::use_navigate;
use server_fn::error::NoCustomError;

use crate::models::ActionFormResp;

#[component]
pub fn ActionFormAlert(
    action_value: RwSignal<Option<Result<ActionFormResp, ServerFnError<NoCustomError>>>>,
    #[prop(into)] error_message: TextProp,
    #[prop(into)] redirect_to: String,
    #[prop(into)] success_message: TextProp,
) -> impl IntoView {
    let navigate = use_navigate();
    let error_msg = move || error_message.clone().get();
    let success_msg = move || success_message.clone().get();

    move || match ActionFormResp::from(action_value).success {
        Some(true) => {
            let navigate = navigate.clone();
            let redirect_to = redirect_to.clone();
            EitherOf3::A(view! {
                <dialog class="modal modal-open">
                    <div class="modal-box">
                        <div>{success_msg.clone()}</div>
                        <div class="modal-action">
                            <button
                                class="btn btn-primary"
                                on:click=move |event| {
                                    event.prevent_default();
                                    navigate(&redirect_to.clone(), Default::default())
                                }
                            >
                                "Ok"
                            </button>
                        </div>
                    </div>
                </dialog>
            })
        }
        Some(false) => EitherOf3::B(view! {
            <div class="pt-2 pb-2">
                <div role="alert" class="alert alert-error">
                    {error_msg.clone()}
                </div>
            </div>
        }),
        _ => EitherOf3::C(()),
    }
}
