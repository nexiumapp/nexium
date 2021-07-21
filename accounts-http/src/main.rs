#[macro_use]
extern crate rocket;

use rand::Rng;
use std::{cmp::min, time::Duration};
use tokio::time::sleep;

mod routes;

/// Start the http server.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let client = rpc_connect("http://[::1]:50051").await;

    rocket::build()
        .mount("/api/accounts", routes::routes())
        .manage(client)
        .launch()
        .await
}

/// Connect to the accounts RPC server.
/// This uses exponential backoff when we are unable to connect (max 60s).
/// It should never fail, only succeed or get stuck in an infinite loop.
async fn rpc_connect(url: &'static str) -> accounts::Client {
    let mut iteration = 1;

    loop {
        match accounts::Client::connect(url).await {
            Ok(client) => return client,
            Err(e) => println!("Failed to connect to Accounts RPC: {}.", e.to_string()),
        }

        let backoff = (min(2 ^ iteration, 60) * 1000) + rand::thread_rng().gen_range(0..1000);
        iteration += 1;

        sleep(Duration::from_millis(backoff)).await;
    }
}
