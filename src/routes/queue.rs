use std::str::FromStr;
use actix_web::{Responder, web};
use serde_json::json;
use crate::comfy_client::ComfyClient;
use crate::domain::ComfyQueueResponse;

pub async fn queue(comfy_client: web::Data<ComfyClient>) -> impl Responder {
    let raw_response = comfy_client
        .get_queue()
        .await
        .expect("Failed to communicate to comfy server");

    let comfy_queue_response = ComfyQueueResponse::from_str(&raw_response)
        .expect("Unexpected response format");

    web::Json(comfy_queue_response.owned_prompts())
}


