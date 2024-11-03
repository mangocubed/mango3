use leptos::prelude::*;

#[component]
pub fn TopBar(children: Children, #[prop(optional, into)] right_items: ViewFnOnce) -> impl IntoView {
    view! {
        <div class="navbar bg-base-300 shadow-md min-h-[52px] h-[52px]">
            <div class="flex-1">{children()}</div>

            <div class="flex-none">{right_items.run()}</div>
        </div>
    }
}
