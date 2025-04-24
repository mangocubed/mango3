mod app;

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
