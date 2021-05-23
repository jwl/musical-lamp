//! src/main.rs

use musical_lamp::configuration::get_configuration;
use musical_lamp::startup::run;
use musical_lamp::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    let subscriber = get_subscriber("musical_lamp".into(), "info".into());
    init_subscriber(subscriber);

    // Load config
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Setup DB
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Setup address to listen to
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // Run app
    run(listener, connection_pool)?.await?;
    Ok(())
}
