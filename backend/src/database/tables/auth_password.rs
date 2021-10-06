use sqlx::{types::Uuid, PgConnection};

use crate::logic::auth::password::AuthPassword;

/// Create a new password for an user account.
pub async fn create(
    conn: &mut PgConnection,
    account: Uuid,
    hash: String,
) -> Result<AuthPassword, sqlx::Error> {
    sqlx::query_as!(
        AuthPassword,
        "INSERT INTO auth_password (account, hash) VALUES ($1, $2) RETURNING *",
        &account,
        &hash
    )
    .fetch_one(conn)
    .await
}

/// Get an authentication method by the account.
pub async fn get(
    conn: &mut PgConnection,
    account: Uuid,
) -> Result<Option<AuthPassword>, sqlx::Error> {
    sqlx::query_as!(
        AuthPassword,
        "SELECT * FROM auth_password WHERE account = $1",
        &account,
    )
    .fetch_optional(conn)
    .await
}
