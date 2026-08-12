pub mod constants;
pub mod engine;
pub mod error;
pub mod server;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn run() -> Result<(), error::Error> {
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
    let _ = match std::env::var("DISCORD_BOT_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'DISCORD_BOT_TOKEN' environment variable".to_string(),
            ));
        }
    };
    server::run().await?;
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
