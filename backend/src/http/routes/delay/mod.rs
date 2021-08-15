use rocket::{serde::json::Json, Route};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

mod error;

/// This is a HTTP route which has a delay before responding.
/// It requires an `seconds` parameter, which is the amount of seconds the service will wait to respond.
#[get("/<seconds>")]
pub async fn delay(seconds: u64) -> Result<Json<DelayResponse>, error::DelayErrors> {
    if seconds < 5 {
        return Err(error::DelayErrors::LowDelay(seconds));
    }

    sleep(Duration::from_secs(seconds)).await;

    Ok(Json(DelayResponse { seconds }))
}

/// Success responses to the client.
#[derive(Serialize)]
pub struct DelayResponse {
    /// The length of the delay in seconds.
    seconds: u64,
}

pub fn routes() -> Vec<Route> {
    routes![delay]
}
