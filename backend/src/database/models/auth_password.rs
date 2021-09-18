use rocket::serde::Serialize;
use sqlx::{types::Uuid, PgConnection};

/// Represents an optional password authentication method for an user account.
#[derive(Debug, Serialize)]
pub struct AuthPassword {
    pub account: Uuid,
    pub hash: String,
}

impl AuthPassword {
    /// Create a new password for an user account.
    pub async fn create(
        conn: &mut PgConnection,
        account: Uuid,
        hash: String,
    ) -> Result<Self, sqlx::Error> {
        let auth = sqlx::query_as!(
            AuthPassword,
            "INSERT INTO auth_password (account, hash) VALUES ($1, $2) RETURNING account, hash",
            &account,
            &hash
        )
        .fetch_one(conn)
        .await?;

        Ok(auth)
    }
}
