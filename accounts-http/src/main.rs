#[macro_use]
extern crate rocket;

mod routes;

/// Start the http server.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let client = accounts::Client::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to the accounts service.");

    rocket::build()
        .mount("/api/accounts", routes::routes())
        .manage(client)
        .launch()
        .await
}
