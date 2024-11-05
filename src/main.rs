use sqlx::MySqlPool;
use zz_rust_web_template::comfy_client::ComfyClient;
use zz_rust_web_template::configuration::get_configuration;
use zz_rust_web_template::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");
    let comfy_client = ComfyClient::new(
        configuration.comfy_address.clone(),
        "Comfy_Rust_Client".to_string(),
    );
    let db_connection_pool = MySqlPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect DB");

    let listener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    run(listener, db_connection_pool, comfy_client)?.await
}
