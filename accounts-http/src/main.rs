extern crate log;
#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

mod routes;

/// Start the http server.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();

    let accounts_host = match std::env::var("NEXIUM_HOST_ACCOUNTS") {
        Ok(host) => host,
        Err(_) => "localhost".to_string(),
    };

    let url = format!("http://{}:50051", accounts_host);
    info!("Connecting to RPC at {}.", url);

    let client = accounts::Client::connect(url)
        .await
        .expect("Failed to connect to the accounts service.");

    let figment = rocket::Config::figment()
        .merge(("address", Ipv4Addr::UNSPECIFIED))
        .merge(("ident", "Nexium Accounts"));

    info!("Starting Rocket service");

    rocket::custom(figment)
        .mount("/api/accounts", routes::routes())
        .manage(client)
        .launch()
        .await
}
