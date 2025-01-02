use leptos::prelude::*;

use crate::context::use_basic_config;

#[component]
pub fn LoadingOverlay(
    #[prop(default = "")] icon_class: &'static str,
    #[prop(optional)] icon_url: Option<String>,
    #[prop(default = "rounded-full")] pulse_class: &'static str,
) -> impl IntoView {
    let basic_config = use_basic_config();
    let is_done = RwSignal::new(false);

    Effect::new(move || is_done.set(true));

    let loading_icon_src = move || icon_url.clone().unwrap_or_else(|| basic_config.asset_url("icon.svg"));

    view! {
        <div class="loading-overlay" class:is-done=is_done>
            <figure>
                <div class=move || format!("pulse {pulse_class}")></div>
                <img class=icon_class src=loading_icon_src />
            </figure>
        </div>
    }
}
