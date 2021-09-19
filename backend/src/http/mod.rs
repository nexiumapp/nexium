use sqlx::{Pool, Postgres};
use std::net::Ipv4Addr;

use crate::environment::Environment;

mod routes;

/// Start the http server.
pub async fn start(conn: Pool<Postgres>, env: Environment) {
    let figment = rocket::Config::figment()
        .merge(("address", Ipv4Addr::UNSPECIFIED))
        .merge(("ident", "Nexium"));

    info!("Starting Rocket service");

    rocket::custom(figment)
        .mount("/api/account", routes::account::routes())
        .manage(conn)
        .manage(env)
        .launch()
        .await
        .expect("Could not start HTTP service.");
}
