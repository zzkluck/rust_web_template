use crate::comfy_client::ComfyClient;
use crate::domain::ComfyQueueResponse;
use actix_web::{web, Responder};
use std::str::FromStr;

pub async fn queue(comfy_client: web::Data<ComfyClient>) -> impl Responder {
    let raw_response = comfy_client
        .get_queue()
        .await
        .expect("Failed to communicate to comfy server");

    let comfy_queue_response =
        ComfyQueueResponse::from_str(&raw_response).expect("Unexpected response format");

    web::Json(comfy_queue_response.owned_prompts())
}
