#[cfg(feature = "ssr")]
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use mango3_leptos_utils::{leptos_http_server, shell};

#[cfg(feature = "ssr")]
use mango3_my_account::app::App;

#[cfg(feature = "ssr")]
fn shell_fn(leptos_options: LeptosOptions) -> impl IntoView {
    shell(leptos_options, App)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::config::get_configuration;

    let leptos_options = get_configuration(None).unwrap().leptos_options;

    leptos_http_server(leptos_options, App, shell_fn).await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
