use std::net::Ipv4Addr;

use sqlx::{Pool, Postgres};

mod routes;

/// Start the http server.
pub async fn start(conn: Pool<Postgres>) {
    let figment = rocket::Config::figment()
        .merge(("address", Ipv4Addr::UNSPECIFIED))
        .merge(("ident", "Nexium"));

    info!("Starting Rocket service");

    rocket::custom(figment)
        .mount("/api/account", routes::account::routes())
        .mount("/api/delay", routes::delay::routes())
        .manage(conn)
        .launch()
        .await
        .expect("Could not start HTTP service.");
}
