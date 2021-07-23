#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

mod routes;

/// Start the http server.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let client = accounts::Client::connect("http://localhost:50051")
        .await
        .expect("Failed to connect to the accounts service.");

    let figment = rocket::Config::figment()
        .merge(("address", Ipv4Addr::UNSPECIFIED))
        .merge(("ident", "Nexium Accounts"));

    rocket::custom(figment)
        .mount("/api/accounts", routes::routes())
        .manage(client)
        .launch()
        .await
}
