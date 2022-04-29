// ! src/main.rs

use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    let port = address.local_addr().unwrap().port();

    // call the run() method in lib.rs and await for the connection
    // the actual "running" process is in here
    println!("Run server on {}", port);
    run(address)?.await
}
