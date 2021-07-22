use accounts::error::PingError;
use rocket::{serde::json::Json, Route, State};
use serde::Serialize;
use serde_json::{json, Value};

/// This is a HTTP route which sends a ping request to the accounts service.
/// It requires an `seconds` parameter, which is the amount of seconds the service will wait to respond.
#[get("/<seconds>")]
async fn delay(rpc: &State<accounts::Client>, seconds: u64) -> Result<Json<DelayResponse>, Value> {
    let res = rpc.ping(seconds).await;

    if let Err(e) = res {
        return Err(match e {
            PingError::DelayTooLowError => {
                json!({ "code": DelayErrors::Validation, "message": "The delay needs to be at least 5 seconds long." })
            }
            PingError::RpcError(_) => {
                json!({ "code": DelayErrors::Rpc, "message": "Service communication error" })
            }
        });
    }

    Ok(Json(DelayResponse { seconds }))
}

/// Possible error codes returned to the client.
#[derive(Serialize)]
enum DelayErrors {
    Validation,
    Rpc,
}

/// Success responses to the client.
#[derive(Serialize)]
struct DelayResponse {
    seconds: u64,
}

/// A list of all routes to mount.
pub fn routes() -> Vec<Route> {
    routes![delay]
}
