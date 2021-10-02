use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::Serialize;
use sqlx::PgConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::{database, logic::account::Account};

pub mod access;
pub mod refresh;

/// This represents an session linked to an account.
#[derive(Debug, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub account: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub creator_ip: IpNetwork,
}

impl Session {
    /// Create a new session for an account.
    pub async fn create(
        conn: &mut PgConnection,
        account: &Account,
        addr: &SocketAddr,
    ) -> Result<Self, CreateError> {
        let ip = IpNetwork::new(
            addr.ip(),
            match addr.ip() {
                std::net::IpAddr::V4(_) => 32,
                std::net::IpAddr::V6(_) => 128,
            },
        )
        .map_err(|_| CreateError::IpParseError)?;

        let session = database::session::new(conn, account.id, ip)
            .await
            .map_err(CreateError::DatabaseError)?;

        Ok(session)
    }

    // Update the last seen date to the current date.
    pub async fn update_seen(&mut self, conn: &mut PgConnection) -> Result<(), sqlx::Error> {
        self.last_seen = Utc::now();

        database::session::update(conn, self).await
    }
}

/// Possible erros when creating a new session.
#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Failed to parse the IP address")]
    IpParseError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}
