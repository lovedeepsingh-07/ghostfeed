use crate::{constants, error};

#[derive(Debug)]
pub struct Engine {
    client: reqwest::Client,
    access_token: String,
    base_url: String,
}

impl Engine {
    pub fn new(access_token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            access_token: access_token.to_string(),
            base_url: format!(
                "{}/{}",
                constants::BASE_URL,
                constants::INSTAGRAM_API_VERSION
            ),
        }
    }

    async fn get_json(&self, url: &str) -> Result<serde_json::Value, error::Error> {
        let res: serde_json::Value = self.client.get(url).send().await?.json().await?;
        Ok(res)
    }

    pub async fn get_convo_list(&self) -> Result<Vec<String>, error::Error> {
        let convo_url = format!(
            "{}/me/conversations?platform=instagram&access_token={}",
            self.base_url, self.access_token
        );
        let convo_url_res: serde_json::Value = self.get_json(&convo_url).await?;
        let convo_list: Vec<serde_json::Value> = convo_url_res["data"]
            .as_array()
            .ok_or(error::Error::DeserializeError(
                "Invalid or missing convo_list[data]".to_string(),
            ))?
            .to_vec();
        let output = convo_list
            .into_iter()
            .filter_map(|obj| {
                obj.get("id")
                    .and_then(|id| id.as_str())
                    .map(|id| id.to_string())
            })
            .collect();
        Ok(output)
    }

    pub async fn get_message_list(
        &self,
        convo_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<serde_json::Value>, error::Error> {
        let mut output: Vec<serde_json::Value> = Vec::new();

        let message_limit = match limit {
            Some(out) => out,
            None => constants::MESSAGE_FETCH_LIMIT,
        };
        let mut next_url = format!(
            "{}/{}/messages?limit={}&fields=from,to,message&access_token={}",
            self.base_url, convo_id, message_limit, self.access_token
        );

        loop {
            let message_list_json: serde_json::Value = self.get_json(&next_url).await?;
            let message_list =
                message_list_json["data"]
                    .as_array()
                    .ok_or(error::Error::DeserializeError(
                        "Invalid or missing messages[data]".to_string(),
                    ))?;
            output.extend(message_list.clone());
            if message_list.len() < constants::MESSAGE_FETCH_LIMIT {
                break;
            }
            next_url = message_list_json["paging"]["next"]
                .as_str()
                .ok_or(error::Error::DeserializeError(
                    "Invalid or missing messages[paging][next]".to_string(),
                ))?
                .to_string();
        }

        Ok(output)
    }
}
