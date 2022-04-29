// ! tests/health_check.rs

use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random PORT");
    let port = listener.local_addr().unwrap().port();

    // create a fake instance of server inside run() method
    let server = zero2prod::run(listener).expect("Failed to bind PORT address");

    // run the server as a background task without waiting for completion
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
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
