//! tests/health_check.rs

use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange/Setup
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve port assigned to us by OS
    let port = listener.local_addr().unwrap().port();
    let server = musical_lamp::run(listener).expect("Failed to bind address");

    // Launch server as a background task
    let _ = tokio::spawn(server);

    // Return application address to the caller
    format!("http://127.0.0.1:{}", port)
}
