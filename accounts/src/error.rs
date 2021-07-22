use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors the ping request might return.
#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum PingError {
    #[error("Delay needs to be at least 5 seconds")]
    DelayTooLowError,
    #[error("{0}")]
    RpcError(String),
}

impl From<PingError> for tonic::Status {
    /// Encode the error into a tonic status.
    fn from(err: PingError) -> tonic::Status {
        tonic::Status::with_details(
            tonic::Code::Internal,
            err.to_string(),
            serde_json::to_string(&err).unwrap().into(),
        )
    }
}

impl From<tonic::Status> for PingError {
    /// Convert a Tonic state into an ping error.
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::Internal => {
                serde_json::from_slice(status.details()).unwrap_or_else(|_| {
                    crate::error::PingError::RpcError("Failed to decode message.".into())
                })
            }
            _ => crate::error::PingError::RpcError(status.to_string()),
        }
    }
}
