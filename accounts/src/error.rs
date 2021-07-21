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

impl From<tonic::Status> for PingError {
    /// Convert a Tonic state into an ping error.
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::Internal => serde_json::from_slice(status.details()).unwrap_or(
                crate::error::PingError::RPCError("Failed to decode message.".into()),
            ),
            _ => crate::error::PingError::RPCError(status.to_string()),
        }
    }
}
