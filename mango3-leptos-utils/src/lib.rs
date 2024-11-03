#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
#[cfg(feature = "ssr")]
use tokio::net::TcpListener;

#[cfg(feature = "ssr")]
use mango3_core::config::load_config;
#[cfg(feature = "ssr")]
use mango3_core::CoreContext;

pub mod components;
pub mod context;
pub mod icons;
pub mod models;
pub mod pages;
pub mod server_functions;

#[cfg(feature = "ssr")]
pub fn shell<F, IV>(options: LeptosOptions, app_fn: F) -> impl IntoView
where
    F: Fn() -> IV + Clone + Copy + Send + 'static,
    IV: IntoView + 'static,
{
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="dark:bg-neutral-950 bg-slate-50">{app_fn()}</body>
        </html>
    }
}

#[cfg(feature = "ssr")]
pub async fn leptos_http_server<F, IV1, IV2>(
    leptos_options: LeptosOptions,
    app_fn: F,
    shell_fn: fn(LeptosOptions) -> IV2,
) where
    F: Fn() -> IV1 + Clone + Copy + Send + 'static,
    IV1: IntoView + 'static,
    IV2: IntoView + 'static,
{
    load_config();

    let core_context = CoreContext::setup().await;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app_fn);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let core_context = core_context.clone();
                move || provide_context(core_context.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell_fn(leptos_options.clone())
            },
        )
        .fallback(file_and_error_handler(shell_fn))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
