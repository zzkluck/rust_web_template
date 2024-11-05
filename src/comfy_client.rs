use crate::domain::workflow::Workflow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ComfyClient {
    http_client: Client,
    base_url: String,
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct ComfyPostPromptResponse {
    pub prompt_id: String,
    pub node_errors: Value,
    pub number: Option<i64>,
    pub error: Option<ComfyPostPromptResponseError>,
}

#[derive(Deserialize, Serialize)]
pub struct ComfyPostPromptResponseError {
    pub type_: String,
    pub message: String,
    pub details: String,
}

impl ComfyClient {
    pub fn new(base_url: String, client_id: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            client_id,
        }
    }

    pub async fn get_queue(&self) -> Result<String, reqwest::Error> {
        self.http_client
            .get(format!("{}/queue", self.base_url))
            .send()
            .await?
            .text()
            .await
    }

    pub async fn post_prompt(
        &self,
        prompt: Workflow,
    ) -> Result<ComfyPostPromptResponse, reqwest::Error> {
        self.http_client
            .post(format!("{}/prompt", self.base_url))
            .json(&prompt)
            .send()
            .await?
            .json::<ComfyPostPromptResponse>()
            .await
    }
}
