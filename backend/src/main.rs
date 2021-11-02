use std::env;

#[macro_use]
extern crate log;
extern crate actix_web;

mod database;
mod environment;
mod http;
mod logic;
mod smtp;

#[actix_web::main]
async fn main() {
    // Set the log level to info by default.
    if let None = env::var_os("RUST_LOG") {
        env::set_var("RUST_LOG", "info,actix_web=error,sqlx=error");
    }

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
    let smtp = smtp::start(db.clone());
    // Start the HTTP server.
    let http = http::start(db, env);

    // Wait for either future to return, then quit.
    tokio::select! {
        _ = smtp => {
            info!("SMTP service exited, goodbye!");
        }
        _ = http => {
            info!("HTTP service exited, goodbye!");
        }
    }
}
