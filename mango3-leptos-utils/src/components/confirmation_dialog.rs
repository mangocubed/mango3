use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn ConfirmationDialog<OA>(children: Children, #[prop(into)] is_open: RwSignal<bool>, on_accept: OA) -> impl IntoView
where
    OA: Fn() + Send + Sync + 'static,
{
    view! {
        <div class="modal" class:modal-open=is_open>
            <div class="modal-box">
                <div>{children()}</div>

                <div class="modal-action">
                    <button class="btn" on:click=move |_| is_open.set(false)>
                        {move || tr!("cancel")}
                    </button>
                    <button
                        class="btn btn-primary"
                        on:click=move |_| {
                            is_open.set(false);
                            on_accept()
                        }
                    >
                        {move || tr!("accept")}
                    </button>
                </div>
            </div>
        </div>
    }
}
