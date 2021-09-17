extern crate log;
#[macro_use]
extern crate rocket;

mod database;
mod http;
mod smtp;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Connect to the database.
    let db = database::connect("postgres://postgres:nexium@localhost/nexium")
        .await
        .expect("Failed to initialize database.");

    // Start the SMTP server.
    tokio::spawn(smtp::start(db.clone()));
    // Start the HTTP server.
    http::start(db).await;
}
