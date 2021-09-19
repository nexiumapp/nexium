use std::net::SocketAddr;

use crate::database::models::{Account, Session};
use ipnetwork::IpNetwork;
use sqlx::PgConnection;
use thiserror::Error;

/// Create a new session for an account.
pub async fn create(
    conn: &mut PgConnection,
    account: &Account,
    addr: &SocketAddr,
) -> Result<Session, CreateError> {
    let ip = IpNetwork::new(
        addr.ip(),
        match addr.ip() {
            std::net::IpAddr::V4(_) => 32,
            std::net::IpAddr::V6(_) => 128,
        },
    )
    .map_err(|_| CreateError::IpParseError)?;

    let session = Session::new(conn, account.id, ip)
        .await
        .map_err(CreateError::DatabaseError)?;

    Ok(session)
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Failed to parse the IP address")]
    IpParseError,
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}
