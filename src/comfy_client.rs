use reqwest::Client;

pub struct ComfyClient {
    http_client: Client,
    base_url: String,
    client_id: String,
}

impl ComfyClient {
    pub fn new(base_url: String, client_id: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            client_id
        }
    }

    pub async fn get_queue(&self) -> Result<String, reqwest::Error> {
        self.http_client
            .get(format!("{}/queue", self.base_url))
            .send().await?
            .text().await
    }
}