use crate::{command, error, engine};
use poise::serenity_prelude as serenity;
use tokio::sync::{mpsc, oneshot};

struct Data {
    pub command_tx: mpsc::Sender<command::Command>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn convo_list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let (response_tx, response_rx) = oneshot::channel::<Vec<engine::Convo>>();
    if let Err(e) = ctx
        .data()
        .command_tx
        .send(command::Command::GetConvoList(response_tx))
        .await
    {
        tracing::error!("shit, {}", e);
    }
    match response_rx.await {
        Ok(convo_list) => {
            let mut res = String::new();
            for (i, curr_convo) in convo_list.iter().enumerate() {
                for p in curr_convo.participants.iter() {
                    res.push_str(&format!("{}: {}", p.username, p.id));
                    if i <= convo_list.len() {
                        res.push('\n');
                    }
                }
            }
            ctx.say(res).await?;
        }
        Err(_) => {
            tracing::error!("failed to do shit");
        }
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn message_list(
    ctx: Context<'_>,
    #[description = "Conversation ID"] convo_id: String,
) -> Result<(), Error> {
    if let Err(e) = ctx
        .data()
        .command_tx
        .send(command::Command::GetMessageList(convo_id))
        .await
    {
        tracing::error!("shit, {}", e);
    }
    ctx.say("yeah a message list").await?;
    Ok(())
}

pub async fn run(command_tx: mpsc::Sender<command::Command>) -> Result<(), error::Error> {
    let _ = command_tx;
    let _ = std::env::var("DISCORD_APP_ID")
        .map_err(|_| error::Error::IOError("'DISCORD_APP_ID' env var missing".to_string()))?;
    let _ = std::env::var("DISCORD_PUBLIC_KEY")
        .map_err(|_| error::Error::IOError("'DISCORD_PUBLIC_KEY' env var missing".to_string()))?;
    let discord_bot_token = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| error::Error::IOError("'DISCORD_BOT_TOKEN' env var missing".to_string()))?;
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
                Ok(Data { command_tx })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(discord_bot_token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
