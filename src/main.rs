//! src/main.rs

use musical_lamp::configuration::get_configuration;
use musical_lamp::startup::run;
use musical_lamp::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    let subscriber = get_subscriber("musical_lamp".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Load config
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Setup DB
    println!("database.connection_string is: {}", &configuration.database.connection_string());
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // Setup address to listen to
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    // Run app
    run(listener, connection_pool)?.await?;
    Ok(())
}
