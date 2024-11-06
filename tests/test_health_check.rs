use reqwest;
use sqlx::MySqlPool;
use tokio;
use zz_rust_web_template::comfy_client::ComfyClient;
use zz_rust_web_template::configuration::get_configuration;
use zz_rust_web_template::domain::RuoyiPromptRequest;
use zz_rust_web_template::startup::run;
use zz_rust_web_template::ExistResponse;

#[tokio::test]
async fn health_check_return_200_and_no_content() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn empty_prompt_id_not_in_query() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/exist?prompt_id={}", address, ""))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success(), "{:?}", response);
    assert_eq!(response.json::<ExistResponse>().await.unwrap().exist, false);
}

#[tokio::test]
async fn example_post_prompt_should_success() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let prompt_request = serde_json::from_str::<RuoyiPromptRequest>(include_str!(
        "../assets/ruoyi_post_prompt.json"
    ))
    .expect("Failed to parse example json");

    let response = client
        .post(&format!("{}/prompt", address))
        .json(&prompt_request)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to read config");
    let comfy_client = ComfyClient::new(
        configuration.comfy_address.clone(),
        "Comfy_Rust_Client".to_string(),
    );
    let db_connection_pool = MySqlPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect DB");

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener, db_connection_pool, comfy_client).expect("Failed to create app");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
