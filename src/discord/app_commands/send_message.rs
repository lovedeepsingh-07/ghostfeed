use super::super::Context;
use crate::{command, error};

#[poise::command(slash_command, prefix_command)]
pub async fn send_message(
    ctx: Context<'_>,
    #[description = "Receiver ID"] recv_id: String,
    #[description = "Message"] message: String,
) -> Result<(), error::Error> {
    ctx.defer().await?;
    if let Err(e) = ctx
        .data()
        .command_tx
        .send(command::Command::SendMessage(recv_id, message))
        .await
    {
        tracing::error!("shit, {}", e);
    }
    ctx.say("sent a message").await?;
    Ok(())
}
