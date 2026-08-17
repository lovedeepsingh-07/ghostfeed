use crate::error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ConvoParticipant {
    pub id: String,
    pub username: String,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Convo {
    pub id: String,
    pub participants: Vec<ConvoParticipant>,
}

impl TryFrom<&serde_json::Value> for Convo {
    type Error = error::Error;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut output = Convo {
            id: String::new(),
            participants: Vec::new(),
        };
        output.id = value["id"]
            .as_str()
            .ok_or(error::Error::DeserializeError(
                "Invalid or missing convo[id]".to_string(),
            ))?
            .to_string();
        let participants =
            value["participants"]["data"]
                .as_array()
                .ok_or(error::Error::DeserializeError(
                    "Invalid or missing convo[participants][data]".to_string(),
                ))?;
        for curr_p in participants.iter() {
            let mut participant = ConvoParticipant {
                id: String::new(),
                username: String::new(),
            };
            participant.id = curr_p["id"]
                .as_str()
                .ok_or(error::Error::DeserializeError(
                    "Invalid or missing participants[id]".to_string(),
                ))?
                .to_string();
            participant.username = curr_p["username"]
                .as_str()
                .ok_or(error::Error::DeserializeError(
                    "Invalid or missing participants[username]".to_string(),
                ))?
                .to_string();
            output.participants.push(participant);
        }
        Ok(output)
    }
}
