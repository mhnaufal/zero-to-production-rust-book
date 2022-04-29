// ! src/main.rs

use std::{env, net::TcpListener};

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let p = env::var("PORT")
        .unwrap_or_else(|_| 3000.to_string())
        .parse::<u16>()
        .unwrap();

    let dynamic_address = format!("0.0.0.0:{}", p);

    let address = TcpListener::bind(dynamic_address)?;
    let port = address.local_addr().unwrap().port();

    // call the run() method in lib.rs and await for the connection
    // the actual "running" process is in here
    println!("Run server on {}", port);
    run(address)?.await
}
