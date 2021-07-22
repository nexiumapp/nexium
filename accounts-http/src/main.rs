extern crate log;
#[macro_use]
extern crate rocket;

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

    info!("Starting Rocket service");

    rocket::build()
        .mount("/api/accounts", routes::routes())
        .manage(client)
        .launch()
        .await
}
