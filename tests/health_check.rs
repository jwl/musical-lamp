//! tests/health_check.rs

#[actix_rt::test]
async fn health_check_works() {
    // Arrange/Setup
    spawn_app();

    // Bring in reqwest to perform HTTP requests against our app
    // Use `cargo add reqwest --dev --vers 0.11` to add under
    // `[dev-dependencies]` in Cargo.toml
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


fn spawn_app() {
    let server = musical_lamp::run().expect("Failed to bind address");

    // Launch server as a background task
    let _ = tokio::spawn(server);
}
