// TODO: Reduce this recursion limit
#![recursion_limit = "160"]

pub mod app;
pub mod components;
pub mod constants;
pub mod context;
pub mod pages;
pub mod presenters;
pub mod server_functions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(app::App);
}
