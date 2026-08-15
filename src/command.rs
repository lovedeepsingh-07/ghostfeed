use crate::error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Command {
    Message(serde_json::Value),
}

impl TryFrom<serde_json::Value> for Command {
    type Error = error::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        tracing::info!("{}", serde_json::to_string_pretty(&value).unwrap());
        let field = value["entry"][0]["changes"][0]["field"].as_str().ok_or(
            error::Error::DeserializeError(
                "Failed to get the [entry][0][changes][0][field] from webhook event".to_string(),
            ),
        )?;
        match field {
            "messages" => {
                tracing::info!("it is a message");
            }
            _ => {
                return Err(error::Error::InvalidInputError(
                    "Invalid field in webhook event".to_string(),
                ));
            }
        }
        Ok(Self::Message(value))
    }
}
