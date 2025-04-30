mod app;
mod components;
mod pages;
mod routes;
mod server_functions;

use crate::app::app;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    mango3_web_utils::dioxus_server(app).await;
}

#[cfg(feature = "web")]
fn main() {
    mango3_web_utils::dioxus_web(app);
}
