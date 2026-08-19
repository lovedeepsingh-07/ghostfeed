pub mod constants;
pub mod discord;
pub mod engine;
pub mod error;
pub mod state;
pub mod webhook_server;

use std::sync::Arc;
use tokio::{
    sync::broadcast,
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

    let instagram_access_token = std::env::var("INSTAGRAM_ACCESS_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_ACCESS_TOKEN' env var missing".to_string())
    })?;
    let instagram_webhook_token = std::env::var("INSTAGRAM_WEBHOOK_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_WEBHOOK_TOKEN' env var missing".to_string())
    })?;
    let state =
        Arc::new(state::State::new(&instagram_access_token, &instagram_webhook_token).await?);

    run_service(
        &mut services,
        shutdown_tx.clone(),
        "webhook_server",
        webhook_server::run(state.clone()),
    )
    .await?;
    run_service(
        &mut services,
        shutdown_tx.clone(),
        "discord_bot",
        discord::run(state.clone()),
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

    // let db = turso::Builder::new_local("./ghostfeed.db").build().await.unwrap();
    // let conn = db.connect().unwrap();
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS users (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         username TEXT NOT NULL
    //     )",
    //     ()
    // ).await.unwrap();
    // conn.execute("INSERT INTO users (username) VALUES (?)", ("alice",)).await.unwrap();
    // conn.execute("INSERT INTO users (username) VALUES (?)", ("bob",)).await.unwrap();
    // let mut res = conn.query("SELECT * FROM users", ()).await.unwrap();
    // let row = res.next().await.unwrap().unwrap();
    // let value = row.get_value(1).unwrap();
    // tracing::info!("{:#?}", value);
    // let row = res.next().await.unwrap().unwrap();
    // let value = row.get_value(1).unwrap();
    // tracing::info!("{:#?}", value);

    if let Err(e) = run().await {
        tracing::error!("Failed to run, error: {}", e);
    }
}
