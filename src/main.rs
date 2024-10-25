use zz_rust_web_template::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
