use crate::{engine, error};

use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn convo_list(
    ctx: Context<'_>,
    // #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let _ = ctx;
    // let _ = user;
    let access_token = std::env::var("INSTAGRAM_ACCESS_TOKEN").unwrap();
    let engine = engine::Engine::new(&access_token);
    let response = format!(
        "```
        {}
        ```",
        serde_json::to_string_pretty(&engine.get_convo_list().await.unwrap()).unwrap()
    );
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn message_list(
    ctx: Context<'_>,
    #[description = "Conversation ID"] convo_id: String,
) -> Result<(), Error> {
    let _ = ctx;
    let access_token = std::env::var("INSTAGRAM_ACCESS_TOKEN").unwrap();
    let engine = engine::Engine::new(&access_token);
    tracing::info!("{}", serde_json::to_string_pretty(&engine.get_message_list(&convo_id, None).await.unwrap()).unwrap());
    Ok(())
}

pub async fn run(discord_bot_token: &str) -> Result<(), error::Error> {
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::DIRECT_MESSAGES;

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![convo_list(), message_list()],
            event_handler: |_, _, _, _| Box::pin(async move { Ok(()) }),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_bot_token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
