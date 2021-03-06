// ! tests/health_check.rs

use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random PORT");
    let port = listener.local_addr().unwrap().port();

    // create a fake instance of server inside run() method
    let server = zero2prod::run(listener).expect("Failed to bind PORT address");

    // run the server as a background task without waiting for completion
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port) // don't forget the "http://"
}

#[actix_web::test]
async fn health_check_works() {
    /* Arrange */
    let address = spawn_app();
    let client = reqwest::Client::new();

    /* Act */
    let response = client
        .get(&format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute Request");

    /* Assert */
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_data_form() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute the request!");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // this one is called `table-driven test` or `parametrised test`
    // hanndled bad inputs in one logic instead of duplicating it
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute the request!");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with code 400 Bad Request when payload is {}",
            error_message
        );
    }
}
