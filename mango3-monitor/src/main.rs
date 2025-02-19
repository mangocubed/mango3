use std::time::Duration;

use apalis::layers::{ErrorHandlingLayer, WorkerBuilderExt};
use apalis::prelude::{Event, Monitor, WorkerBuilder, WorkerFactoryFn};
use log::{error, info};

use mango3_core::config::load_config;
use mango3_core::CoreContext;

use tokio::signal::unix::SignalKind;

mod constants;
mod workers;

use crate::workers::{admin_mailer_worker, guest_mailer_worker, mailer_worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_config();

    info!("Monitor starting");

    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt())?;
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    let core_context = CoreContext::setup().await;

    let admin_mailer_worker = WorkerBuilder::new("admin-mailer")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(core_context.jobs.storage_admin_mailer.clone())
        .build_fn(admin_mailer_worker);

    let guest_mailer_worker = WorkerBuilder::new("guest-mailer")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(core_context.jobs.storage_guest_mailer.clone())
        .build_fn(guest_mailer_worker);

    let mailer_worker = WorkerBuilder::new("mailer")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .concurrency(2)
        .backend(core_context.jobs.storage_mailer.clone())
        .build_fn(mailer_worker);

    Monitor::new()
        .register(admin_mailer_worker)
        .register(guest_mailer_worker)
        .register(mailer_worker)
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Engage(task_id) => {
                    info!("Worker [{worker_id}] got a job with id: {task_id}");
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
                _ => {}
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
