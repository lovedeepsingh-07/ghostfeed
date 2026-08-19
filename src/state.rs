use crate::{engine, error};

#[derive(Debug)]
pub struct State {
    pub engine: engine::Engine,
}

impl State {
    pub async fn new(
        instagram_access_token: &str,
        instagram_webhook_token: &str,
    ) -> Result<Self, error::Error> {
        let engine = engine::Engine::new(instagram_access_token, instagram_webhook_token).await?;
        Ok(Self { engine })
    }
}
