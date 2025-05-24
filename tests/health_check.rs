use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // we need to bring in reqwest
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Faild to execute request.");

    print!("RESPONSE: {}", response.status());
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bound random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Faild to bind address.");

    // zero2prod::run()
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
