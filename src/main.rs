//! src/main.rs

use env_logger::Env;
use musical_lamp::configuration::get_configuration;
use musical_lamp::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read config
    let configuration = get_configuration().expect("Failed to read configuration.");

    // init logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
