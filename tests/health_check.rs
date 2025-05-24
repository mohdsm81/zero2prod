#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // we need to bring in reqwest
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Faild to execute request.");

    print!("RESPONSE: {}", response.status());
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Faild to bind address.");

    // zero2prod::run()
    let _ = tokio::spawn(server);
}
