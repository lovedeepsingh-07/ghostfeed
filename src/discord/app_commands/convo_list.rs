use super::super::Context;
use crate::error;

#[poise::command(slash_command, prefix_command)]
pub async fn convo_list(ctx: Context<'_>) -> Result<(), error::Error> {
    ctx.defer().await?;

    let state = ctx.data().state.clone();
    let convo_list = state.engine.get_convo_list().await?;
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
    Ok(())
}
