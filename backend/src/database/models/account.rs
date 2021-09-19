use rocket::serde::Serialize;
use sqlx::{types::Uuid, PgConnection};

/// Representing an account of an user.
#[derive(Debug, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
}

impl Account {
    /// Create a new user account.
    pub async fn create(conn: &mut PgConnection, username: &str) -> Result<Self, sqlx::Error> {
        let account = sqlx::query_as!(
            Account,
            "INSERT INTO account (username) VALUES ($1) RETURNING id, username",
            &username,
        )
        .fetch_one(conn)
        .await?;

        Ok(account)
    }

    /// Find an account by username.
    pub async fn find_username(
        conn: &mut PgConnection,
        username: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let account = sqlx::query_as!(
            Account,
            "SELECT id, username FROM account WHERE username = $1",
            &username,
        )
        .fetch_optional(conn)
        .await?;

        Ok(account)
    }
}
