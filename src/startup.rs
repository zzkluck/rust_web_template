use super::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use super::configuration::Settings;

pub fn run(listener: TcpListener, configuration: Settings) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move ||
            App::new()
                .route("/health_check", web::get().to(health_check))
                .app_data(configuration.clone())
    )
        .listen(listener)?
        .run();
    Ok(server)
}
