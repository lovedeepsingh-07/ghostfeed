use super::super::Context;
use crate::error;

#[poise::command(slash_command, prefix_command)]
pub async fn message_list(
    ctx: Context<'_>,
    #[description = "Conversation ID"] convo_id: String,
) -> Result<(), error::Error> {
    ctx.defer().await?;

    let data = ctx.data();
    data.state.engine.get_message_list(&convo_id, None).await?;

    ctx.say("yeah a message list").await?;
    Ok(())
}
