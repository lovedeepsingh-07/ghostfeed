pub mod convo;

use crate::{constants, error};
pub use convo::{Convo, ConvoParticipant};
use reqwest as req;

#[derive(Debug)]
pub struct Engine {
    client: req::Client,
    access_token: String,
    base_url: String,
    user_id: String,
}

async fn get_json(client: &req::Client, url: &str) -> Result<serde_json::Value, error::Error> {
    let res: serde_json::Value = client.get(url).send().await?.json().await?;
    Ok(res)
}
async fn post_json(
    client: &req::Client,
    url: &str,
    headers: req::header::HeaderMap,
    body: &serde_json::Value,
) -> Result<serde_json::Value, error::Error> {
    let res: serde_json::Value = client
        .post(url)
        .headers(headers)
        .json(body)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

impl Engine {
    pub async fn new(access_token: &str) -> Result<Self, error::Error> {
        let client = reqwest::Client::new();
        let base_url = format!(
            "{}/{}",
            constants::BASE_URL,
            constants::INSTAGRAM_API_VERSION
        );
        let user_id_json: serde_json::Value = get_json(
            &client,
            &format!("{}/me?access_token={}", base_url, access_token),
        )
        .await?;
        let user_id = user_id_json["id"]
            .as_str()
            .ok_or(error::Error::DeserializeError(
                "Invalid or missing user[id]".to_string(),
            ))?
            .to_string();

        Ok(Self {
            client,
            access_token: access_token.to_string(),
            base_url,
            user_id,
        })
    }

    pub async fn get_convo_list(&self) -> Result<Vec<convo::Convo>, error::Error> {
        let convo_url = format!(
            "{}/me/conversations?platform=instagram&access_token={}&fields=participants",
            self.base_url, self.access_token
        );
        let convo_url_res: serde_json::Value = get_json(&self.client, &convo_url).await?;
        let convo_list: Vec<serde_json::Value> = convo_url_res["data"]
            .as_array()
            .ok_or(error::Error::DeserializeError(
                "Invalid or missing convo_list[data]".to_string(),
            ))?
            .to_vec();
        let mut output = Vec::new();
        for curr_convo in convo_list.iter() {
            output.push(convo::Convo::try_from(curr_convo)?);
        }
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
            let message_list_json: serde_json::Value = get_json(&self.client, &next_url).await?;
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

    pub async fn send_message(&self, recv_id: &str, message: &str) -> Result<(), error::Error> {
        let url = format!("{}/{}/messages", self.base_url, self.user_id);
        let mut headers = req::header::HeaderMap::new();
        headers.insert(
            req::header::AUTHORIZATION,
            req::header::HeaderValue::from_str(&format!("Bearer {}", self.access_token))?,
        );
        headers.insert(
            req::header::CONTENT_TYPE,
            req::header::HeaderValue::from_str("application/json")?,
        );
        let body = serde_json::json!({
            "recipient": {
                "id": recv_id
            },
            "message": {
                "text": message
            }
        });
        let res = post_json(&self.client, &url, headers, &body).await?;
        tracing::info!("{}", serde_json::to_string_pretty(&res)?);
        Ok(())
    }
}
