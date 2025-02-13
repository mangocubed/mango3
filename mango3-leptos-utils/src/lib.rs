#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_meta::HashedStylesheet;

#[macro_use]
pub mod macros;

pub mod components;
pub mod constants;
pub mod context;
pub mod icons;
pub mod utils;

pub mod models;
pub mod pages;
pub mod server_functions;

#[cfg(feature = "ssr")]
pub mod ssr;

leptos_i18n::load_locales!();

#[cfg(feature = "ssr")]
pub fn shell<F, IV>(options: LeptosOptions, app_fn: F, body_class: Option<&'static str>) -> impl IntoView
where
    F: Fn() -> IV + Clone + Copy + Send + 'static,
    IV: IntoView + 'static,
{
    use leptos_meta::MetaTags;

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
                <HashedStylesheet id="leptos" options=options />
            </head>
            <body class=body_class>{app_fn()}</body>
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
    use std::net::SocketAddr;
    use std::str::FromStr;

    use axum::Router;
    use axum_client_ip::SecureClientIpSource;
    use cookie::{Key, SameSite};
    use fred::prelude::{ClientLike, Config, Pool};
    use leptos::logging::log;
    use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};
    use time::Duration;
    use tokio::net::TcpListener;
    use tower_sessions::{Expiry, SessionManagerLayer};
    use tower_sessions_redis_store::RedisStore;

    use mango3_core::config::{load_config, BASIC_CONFIG, MISC_CONFIG, SESSIONS_CONFIG};
    use mango3_core::CoreContext;

    load_config();

    let core_context = CoreContext::setup().await;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(app_fn);
    let redis_pool = Pool::new(
        Config::from_url(&SESSIONS_CONFIG.redis_url).expect("Could not get Redis URL for session."),
        None,
        None,
        None,
        10,
    )
    .expect("Could not get Redis pool for session.");

    let redis_conn = redis_pool.connect();
    redis_pool
        .wait_for_connect()
        .await
        .expect("Could not get Redis connection for session.");

    let session_store = RedisStore::new(redis_pool);
    let session_layer = SessionManagerLayer::new(session_store)
        .with_domain(BASIC_CONFIG.domain.clone())
        .with_expiry(Expiry::OnInactivity(Duration::days(30)))
        .with_http_only(true)
        .with_name("_mango3_session")
        .with_private(Key::from(SESSIONS_CONFIG.key.as_bytes()))
        .with_same_site(SameSite::Strict)
        .with_secure(BASIC_CONFIG.secure);
    let client_ip_source_layer = SecureClientIpSource::from_str(&MISC_CONFIG.client_ip_source)
        .expect("Could not get client IP source.")
        .into_extension();

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
        .layer(session_layer)
        .layer(client_ip_source_layer)
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    redis_conn.await.unwrap().unwrap();
}
