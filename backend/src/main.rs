extern crate log;
#[macro_use]
extern crate rocket;

mod database;
mod environment;
mod http;
mod logic;
mod smtp;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env = match environment::get() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to get configuration: {}", e);
            return;
        }
    };

    // Connect to the database.
    let db = database::connect(env.database_url.as_str())
        .await
        .expect("Failed to initialize database.");

    // Start the SMTP server.
    tokio::spawn(smtp::start(db.clone()));
    // Start the HTTP server.
    http::start(db, env).await;
}
