use leptos::prelude::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex">
            <span class="loading loading-spinner loading-lg m-auto"></span>
        </div>
    }
}
