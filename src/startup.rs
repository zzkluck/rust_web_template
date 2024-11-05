use super::routes::queue;
use super::routes::{health_check, prompt, prompt_in_query};
use crate::comfy_client::ComfyClient;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::MySqlPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    comfy_client: ComfyClient,
) -> Result<Server, std::io::Error> {
    let comfy_client = Data::new(comfy_client);
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/queue", web::get().to(queue))
            .route("/exist", web::get().to(prompt_in_query))
            .route("/prompt", web::post().to(prompt))
            .app_data(comfy_client.clone())
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
