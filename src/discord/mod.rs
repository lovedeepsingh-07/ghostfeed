pub mod app_commands;
pub mod setup;

use crate::{error, state};
use poise::serenity_prelude as serenity;
use std::sync::Arc;

pub struct Data {
    pub state: Arc<state::State>,
}
pub type Context<'a> = poise::Context<'a, Data, error::Error>;

pub async fn run(state: Arc<state::State>) -> Result<(), error::Error> {
    let _ = std::env::var("DISCORD_APP_ID")
        .map_err(|_| error::Error::IOError("'DISCORD_APP_ID' env var missing".to_string()))?;
    let _ = std::env::var("DISCORD_PUBLIC_KEY")
        .map_err(|_| error::Error::IOError("'DISCORD_PUBLIC_KEY' env var missing".to_string()))?;
    let discord_bot_token = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| error::Error::IOError("'DISCORD_BOT_TOKEN' env var missing".to_string()))?;
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::DIRECT_MESSAGES;

    let framework = poise::Framework::<Data, error::Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                app_commands::convo_list(),
                app_commands::message_list(),
                app_commands::send_message(),
            ],
            event_handler: |_, _, _, _| Box::pin(async move { Ok(()) }),
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move { setup::run(state, ctx, ready, framework).await })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_bot_token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
