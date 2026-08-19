use super::Data;
use crate::{constants, error, state};
use poise::serenity_prelude as serenity;
use std::sync::Arc;

pub async fn run(
    state: Arc<state::State>,
    ctx: &serenity::Context,
    ready: &serenity::Ready,
    framework: &poise::Framework<Data, error::Error>,
) -> Result<Data, error::Error> {
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

    let guild = ready.guilds.first().ok_or(error::Error::SerenityError(
        "Bot is not installed in any guilds".to_string(),
    ))?;

    let c_list = guild.id.channels(&ctx.http).await?;
    for (_, curr_c) in c_list {
        if curr_c.name == constants::DISCORD_MOD_CHANNEL_NAME {
            continue;
        }
        let _ = ctx
            .http
            .delete_channel(curr_c.id, Some("okay what is this fucking reason"))
            .await?;
    }

    let convo_list = state.engine.get_convo_list().await?;
    for curr_convo in convo_list.iter() {
        tracing::info!("{:#?}", curr_convo.id);
        let c = ctx
            .http
            .create_channel(
                guild.id,
                &serde_json::json!({
                    "name": curr_convo.id
                }),
                Some("what is this fucking reason"),
            )
            .await?;
        let mut res = String::new();
        for (i, p) in curr_convo.participants.iter().enumerate() {
            res.push_str(&format!("{}: ({}, {}) ", p.username, p.id, curr_convo.id));
            if i <= curr_convo.participants.len() {
                res.push('\n');
            }
        }
        c.say(&ctx.http, res).await?;
    }

    Ok(Data { state })
}
