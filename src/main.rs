pub mod command;
pub mod constants;
pub mod error;
pub mod orchestrator;
pub mod webhook_server;

use tokio::{
    sync::{broadcast, mpsc},
    task::{self, JoinSet},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub enum TaskResultKind {
    Completed,
    Failed(error::Error),
    Shutdown,
}
pub struct TaskResult(String, TaskResultKind);

pub async fn run_service<F>(
    services: &mut JoinSet<TaskResult>,
    shutdown_tx: broadcast::Sender<()>,
    service_name: &'static str,
    service_future: F,
) -> Result<(), error::Error>
where
    F: Future<Output = Result<(), error::Error>> + Send + 'static,
{
    let mut shutdown_rx = shutdown_tx.subscribe();
    services.spawn(async move {
        tracing::info!("Starting service: {}", service_name);
        tokio::select! {
            res = service_future => {
                let _ = shutdown_tx.send(());
                if let Err(e) = res {
                    return TaskResult(service_name.to_string(), TaskResultKind::Failed(e));
                }
                return TaskResult(service_name.to_string(), TaskResultKind::Completed);
            }
            _ = shutdown_rx.recv() => {
                return TaskResult(service_name.to_string(), TaskResultKind::Shutdown);
            }
        }
    });
    Ok(())
}

pub fn handle_service_result(result: Result<TaskResult, task::JoinError>) {
    match result {
        Ok(TaskResult(service_name, TaskResultKind::Completed)) => {
            tracing::info!("{} service completed without failure", service_name);
        }
        Ok(TaskResult(service_name, TaskResultKind::Failed(e))) => {
            tracing::error!("{} service failed with error: {}", service_name, e);
        }
        Ok(TaskResult(service_name, TaskResultKind::Shutdown)) => {
            tracing::info!("{} service shutdown", service_name);
        }
        Err(e) => {
            tracing::error!("Service task paniched: {}", e);
        }
    }
}

async fn run() -> Result<(), error::Error> {
    let (shutdown_tx, _) = broadcast::channel::<()>(16);
    let mut services: JoinSet<TaskResult> = JoinSet::new();

    let (command_tx, command_rx) = mpsc::channel::<command::Command>(1024);

    run_service(
        &mut services,
        shutdown_tx.clone(),
        "orchestrator",
        orchestrator::run(command_rx),
    )
    .await?;
    run_service(
        &mut services,
        shutdown_tx.clone(),
        "webhook_server",
        webhook_server::run(command_tx.clone()),
    )
    .await?;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Shutting down due to ctrl-c");
                let _ = shutdown_tx.send(());
            }
            Some(result) = services.join_next() => {
                let _ = shutdown_tx.send(());
                handle_service_result(result);
            }
        };
        if services.is_empty() {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(
            "ghostfeed=debug,ghostfeed_lib=debug",
        ))
        .init();

    if let Err(e) = run().await {
        tracing::error!("Failed to run, error: {}", e);
    }
}
