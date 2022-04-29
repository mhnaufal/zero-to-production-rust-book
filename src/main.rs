// ! src/main.rs

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // call the run() method in lib.rs and await for the connection
    // actual "run" is in here
    run()?.await
}
