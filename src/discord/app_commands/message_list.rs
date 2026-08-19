use super::super::Context;
use crate::{command, error};

#[poise::command(slash_command, prefix_command)]
pub async fn message_list(
    ctx: Context<'_>,
    #[description = "Conversation ID"] convo_id: String,
) -> Result<(), error::Error> {
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
