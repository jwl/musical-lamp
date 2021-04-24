//! src/main.rs

use std::net::TcpListener;

use musical_lamp::startup::run;
use musical_lamp::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    //let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
