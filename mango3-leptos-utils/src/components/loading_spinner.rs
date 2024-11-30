use leptos::prelude::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex m-4 justify-center">
            <span class="loading loading-spinner loading-lg"></span>
        </div>
    }
}
