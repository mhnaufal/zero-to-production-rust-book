// ! tests/health_check.rs

#[actix_web::test]
async fn health_check_works() {
    /* Arrange */
    spawn_app();
    let client = reqwest::Client::new();

    /* Act */
    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed");

    /* Assert */
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    // create a fake instance of server inside run() method
    let server = zero2prod::run().expect("Failed");
    // run the server as a background task without waiting for completion
    let _ = tokio::spawn(server);
}
