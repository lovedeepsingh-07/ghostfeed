use super::super::Context;
use crate::error;

#[poise::command(slash_command, prefix_command)]
pub async fn send_message(
    ctx: Context<'_>,
    #[description = "Receiver ID"] recv_id: String,
    #[description = "Message"] message: String,
) -> Result<(), error::Error> {
    ctx.defer().await?;

    let data = ctx.data();
    data.state.engine.send_message(&recv_id, &message).await?;

    ctx.say("sent a message").await?;
    Ok(())
}
