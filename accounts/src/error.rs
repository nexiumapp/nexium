use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors the ping request might return.
#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum PingError {
    #[error("Delay needs to be at least 5 seconds")]
    DelayTooLowError,
    #[error("{0}")]
    RPCError(String),
}

impl Into<tonic::Status> for PingError {
    /// Encode the error into a tonic status.
    fn into(self) -> tonic::Status {
        tonic::Status::with_details(
            tonic::Code::Internal,
            self.to_string(),
            serde_json::to_string(&self).unwrap().into(),
        )
    }
}
