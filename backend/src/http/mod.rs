extern crate log;

use std::net::Ipv4Addr;

mod routes;

/// Start the http server.
pub async fn start() {
    let figment = rocket::Config::figment()
        .merge(("address", Ipv4Addr::UNSPECIFIED))
        .merge(("ident", "Nexium"));

    info!("Starting Rocket service");

    rocket::custom(figment)
        .mount("/api/delay", routes::delay::routes())
        .launch()
        .await
        .expect("Could not start HTTP service.");
}
