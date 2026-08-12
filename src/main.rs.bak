pub mod constants;
pub mod engine;
pub mod error;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn run() -> Result<(), error::Error> {
    let access_token = match std::env::var("INSTAGRAM_ACCESS_TOKEN") {
        Ok(out) => out,
        Err(_) => {
            return Err(error::Error::IOError(
                "You need to provide the 'INSTAGRAM_ACCESS_TOKEN' environment variable".to_string(),
            ));
        }
    };
    let engine = engine::Engine::new(&access_token);
    let convo_list = engine.get_convo_list().await?;
    for convo_id in convo_list.iter() {
        let message_list = engine.get_message_list(convo_id, None).await?;
        tracing::info!("{:#?}", message_list);
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
        tracing::error!("failed to run, error: {}", e);
    }
}
