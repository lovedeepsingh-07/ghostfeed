use crate::{command, error};
use tokio::sync::mpsc;

pub async fn run(mut command_rx: mpsc::Receiver<command::Command>) -> Result<(), error::Error> {
    let instagram_webhook_token = std::env::var("INSTAGRAM_WEBHOOK_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_WEBHOOK_TOKEN' env var missing".to_string())
    })?;
    let _ = std::env::var("INSTAGRAM_ACCESS_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_ACCESS_TOKEN' env var missing".to_string())
    })?;
    let _ = std::env::var("DISCORD_APP_ID")
        .map_err(|_| error::Error::IOError("'DISCORD_APP_ID' env var missing".to_string()))?;
    let _ = std::env::var("DISCORD_PUBLIC_KEY")
        .map_err(|_| error::Error::IOError("'DISCORD_PUBLIC_KEY' env var missing".to_string()))?;
    let _ = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| error::Error::IOError("'DISCORD_BOT_TOKEN' env var missing".to_string()))?;

    while let Some(command) = command_rx.recv().await {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        match command {
            command::Command::VerifyInstagramWebhookToken(token_to_verify, response_tx) => {
                if let Err(e) = response_tx.send(token_to_verify == instagram_webhook_token) {
                    tracing::error!(
                        "Failed to send a response from orchestrator for verifying instagram webhook token, {}",
                        e
                    );
                }
            }
            command::Command::Message(_) => {}
        }
    }
    Ok(())
}
