use accounts::error::PingError;
use rocket::{Route, State};
use serde::Serialize;
use serde_json::{json, Value};

/// This is a HTTP route which sends a ping request to the accounts service.
/// It requires an `seconds` parameter, which is the amount of seconds the service will wait to respond.
#[get("/<seconds>")]
async fn delay(rpc: &State<accounts::Client>, seconds: u64) -> Result<(), Value> {
    let res = rpc.ping(seconds).await;

    if let Err(e) = res {
        return Err(match e {
            PingError::DelayTooLowError => {
                json!({ "code": DelayErrors::Validation, "message": "The delay needs to be at least 5 seconds long." })
            }
            PingError::RPCError(_) => {
                json!({ "code": DelayErrors::RPC, "message": "Service communication error" })
            }
        });
    }

    Ok(())
}

/// Possible error codes returned to the client.
#[derive(Serialize)]
enum DelayErrors {
    Validation,
    RPC,
}

/// A list of all routes to mount.
pub fn routes() -> Vec<Route> {
    routes![delay]
}
