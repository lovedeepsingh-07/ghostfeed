use super::super::Context;
use crate::{command, engine, error};
use tokio::sync::oneshot;

#[poise::command(slash_command, prefix_command)]
pub async fn convo_list(ctx: Context<'_>) -> Result<(), error::Error> {
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
                // TODO remove the hardcoded username here
                if let Some(p) = curr_convo
                    .participants
                    .iter()
                    .find(|p| p.username != "iaminthebasement")
                {
                    res.push_str(&format!("{}: ({}, {}) ", p.username, p.id, curr_convo.id));
                }
                if i <= convo_list.len() {
                    res.push('\n');
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
