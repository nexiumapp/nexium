use sqlx::PgConnection;
use uuid::Uuid;

use crate::logic::account::Account;

/// Create a new user account.
pub async fn create(conn: &mut PgConnection, username: &str) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        "INSERT INTO account (username, displayname) VALUES ($1, $2) RETURNING *",
        &username,
        &username
    )
    .fetch_one(conn)
    .await?;

    Ok(account)
}

/// Find an account by id.
pub async fn find(conn: &mut PgConnection, id: &Uuid) -> Result<Option<Account>, sqlx::Error> {
    let account = sqlx::query_as!(Account, "SELECT * FROM account WHERE id = $1", &id,)
        .fetch_optional(conn)
        .await?;

    Ok(account)
}

/// Find an account by username.
pub async fn find_username(
    conn: &mut PgConnection,
    username: &str,
) -> Result<Option<Account>, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        "SELECT * FROM account WHERE username = $1",
        &username,
    )
    .fetch_optional(conn)
    .await?;

    Ok(account)
}
