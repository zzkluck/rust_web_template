use crate::comfy_client::ComfyClient;
use crate::domain::ComfyQueueResponse;
use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct ExistRequest {
    pub prompt_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExistResponse {
    pub exist: bool,
}

pub async fn prompt_in_query(
    query: web::Query<ExistRequest>,
    comfy_client: web::Data<ComfyClient>,
) -> impl Responder {
    let raw_response = comfy_client
        .get_queue()
        .await
        .expect("Failed to communicate to comfy server");

    let comfy_queue_response =
        ComfyQueueResponse::from_str(&raw_response).expect("Unexpected response format");

    web::Json(ExistResponse {
        exist: comfy_queue_response.prompt_exists(&query.prompt_id),
    })
}
