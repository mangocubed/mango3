use std::time::Duration;

use apalis::layers::tracing::TraceLayer;
use apalis::prelude::{Event, Monitor, WorkerBuilder, WorkerFactoryFn};
use apalis::utils::TokioExecutor;
use log::{error, info};

use mango3_core::config::load_config;
use mango3_core::CoreContext;

use tokio::signal::unix::SignalKind;

mod constants;
mod workers;

use crate::workers::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_config();

    env_logger::init();

    info!("Monitor starting");

    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())?;
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    let core_context = CoreContext::setup().await;

    Monitor::<TokioExecutor>::new()
        .register_with_count(
            2,
            WorkerBuilder::new("mailer")
                .layer(TraceLayer::new())
                .with_storage(core_context.jobs.storage_mailer.clone())
                .build_fn(mailer_worker),
        )
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Engage => {
                    info!("Worker [{worker_id}] got a job");
                }
                Event::Error(e) => {
                    error!("Worker [{worker_id}] encountered an error: {e}");
                }

                Event::Exit => {
                    info!("Worker [{worker_id}] exited");
                }
                Event::Idle => {
                    info!("Worker [{worker_id}] is idle");
                }
                Event::Start => {
                    info!("Worker [{worker_id}] started");
                }
                Event::Stop => {
                    info!("Worker [{worker_id}] stopped");
                }
            }
        })
        .shutdown_timeout(Duration::from_millis(5000))
        .run_with_signal(async {
            info!("Monitor started");

            tokio::select! {
                _ = sigint.recv() => info!("Received SIGINT."),
                _ = sigterm.recv() => info!("Received SIGTERM."),
            };

            info!("Monitor shutting down");

            Ok(())
        })
        .await?;

    info!("Monitor shutdown complete");

    Ok(())
}
