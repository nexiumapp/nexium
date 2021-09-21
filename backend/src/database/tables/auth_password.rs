use sqlx::{types::Uuid, PgConnection};

use crate::logic::auth::password::AuthPassword;

/// Create a new password for an user account.
pub async fn create(
    conn: &mut PgConnection,
    account: Uuid,
    hash: String,
) -> Result<AuthPassword, sqlx::Error> {
    let auth = sqlx::query_as!(
        AuthPassword,
        "INSERT INTO auth_password (account, hash) VALUES ($1, $2) RETURNING *",
        &account,
        &hash
    )
    .fetch_one(conn)
    .await?;

    Ok(auth)
}
