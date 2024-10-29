use super::routes::health_check;
use super::routes::queue;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use actix_web::web::Data;
use crate::comfy_client::ComfyClient;

pub fn run(listener: TcpListener, comfy_client: ComfyClient) -> Result<Server, std::io::Error> {
    let comfy_client = Data::new(comfy_client);

    let server = HttpServer::new(move ||
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/queue", web::get().to(queue))
                .app_data(comfy_client.clone())
    )
        .listen(listener)?
        .run();
    Ok(server)
}
