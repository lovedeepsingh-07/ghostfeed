use crate::{engine, error};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Command {
    Message(serde_json::Value),
    GetConvoList(oneshot::Sender<Vec<engine::Convo>>),
    GetMessageList(String),
    VerifyInstagramWebhookToken(String, oneshot::Sender<bool>),
}

pub async fn run_handler(mut command_rx: mpsc::Receiver<Command>) -> Result<(), error::Error> {
    let instagram_webhook_token = std::env::var("INSTAGRAM_WEBHOOK_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_WEBHOOK_TOKEN' env var missing".to_string())
    })?;
    let instagram_access_token = std::env::var("INSTAGRAM_ACCESS_TOKEN").map_err(|_| {
        error::Error::IOError("'INSTAGRAM_ACCESS_TOKEN' env var missing".to_string())
    })?;
    let engine = engine::Engine::new(&instagram_access_token);

    while let Some(command) = command_rx.recv().await {
        match command {
            Command::VerifyInstagramWebhookToken(token_to_verify, response_tx) => {
                if response_tx.send(token_to_verify == instagram_webhook_token).is_err() {
                    tracing::error!("Failed to send a response from orchestrator for verifying instagram webhook token");
                }
            }
            Command::GetConvoList(response_tx) => {
                if response_tx.send(engine.get_convo_list().await?).is_err() {
                    tracing::error!("Failed to send a response from orchestrator for converstaion list");
                };
            }
            Command::GetMessageList(convo_id) => {
                tracing::info!(
                    "message_list: {}",
                    serde_json::to_string_pretty(&engine.get_message_list(&convo_id, None).await?)?
                );
            }
            _ => {}
        }
    }
    Ok(())
}

// impl TryFrom<serde_json::Value> for Command {
//     type Error = error::Error;
//     fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
//         tracing::info!("{}", serde_json::to_string_pretty(&value).unwrap());
//         let field = value["entry"][0]["changes"][0]["field"].as_str().ok_or(
//             error::Error::DeserializeError(
//                 "Failed to get the [entry][0][changes][0][field] from webhook event".to_string(),
//             ),
//         )?;
//         match field {
//             "messages" => {
//                 tracing::info!("it is a message");
//             }
//             _ => {
//                 return Err(error::Error::InvalidInputError(
//                     "Invalid field in webhook event".to_string(),
//                 ));
//             }
//         }
//         Ok(Self::Message(value))
//     }
// }
