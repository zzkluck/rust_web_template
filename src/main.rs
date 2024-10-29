use zz_rust_web_template::configuration::get_configuration;
use zz_rust_web_template::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config.");
    let listener =
        std::net::TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    run(listener, configuration)?.await
}
