use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rocket::serde::Serialize;
use sqlx::{types::ipnetwork::IpNetwork, PgConnection};
use uuid::Uuid;

/// This represents an session linked to an account.
#[derive(Debug, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub secret: String,
    pub account: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub creator_ip: IpNetwork,
}

impl Session {
    /// Create a new session.
    pub async fn new(
        conn: &mut PgConnection,
        account: Uuid,
        ip: IpNetwork,
    ) -> Result<Self, sqlx::Error> {
        let secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(256)
            .map(char::from)
            .collect();

        let auth = sqlx::query_as!(
            Session,
            "INSERT INTO session (secret, account, creator_ip) VALUES ($1, $2, $3) RETURNING id, secret, account, created_at, last_seen, creator_ip",
            &secret,
            &account,
            &ip
        )
        .fetch_one(conn)
        .await?;

        Ok(auth)
    }
}
