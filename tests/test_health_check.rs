use reqwest;
use tokio;

#[tokio::test]
async fn health_check_return_200_and_no_content() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() {
    let server = zz_rust_web_template::startup::run().expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}
