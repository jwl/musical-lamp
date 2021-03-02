//! src/main.rs

use musical_lamp::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
