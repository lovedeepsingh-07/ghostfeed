pub mod bot;
pub mod command;
pub mod constants;
pub mod engine;
pub mod error;
pub mod server;

use tokio::sync::mpsc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn run() -> Result<(), error::Error> {
    let (command_tx, mut command_rx) = mpsc::channel::<command::Command>(constants::COMMAND_CAP);
    let _ = command_tx;

    tokio::spawn(async move {
        while let Some(cmd) = command_rx.recv().await {
            tracing::info!("command: {:#?}", cmd);
        }
    });

    let _ = match std::env::var("INSTAGRAM_ACCESS_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'INSTAGRAM_ACCESS_TOKEN' environment variable".to_string(),
            ));
        }
    };
    let _ = match std::env::var("DISCORD_APP_ID") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'DISCORD_APP_ID' environment variable".to_string(),
            ));
        }
    };
    let _ = match std::env::var("DISCORD_PUBLIC_KEY") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'DISCORD_PUBLIC_KEY' environment variable".to_string(),
            ));
        }
    };
    let discord_bot_token = match std::env::var("DISCORD_BOT_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'DISCORD_BOT_TOKEN' environment variable".to_string(),
            ));
        }
    };
    bot::run(&discord_bot_token).await?;

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
        tracing::error!("failed to run, error: {}", e);
    }
}
