use sqlx::{types::ipnetwork::IpNetwork, PgConnection};
use uuid::Uuid;

use crate::logic::session::Session;

/// Create a new session.
pub async fn new(
    conn: &mut PgConnection,
    account: Uuid,
    ip: IpNetwork,
) -> Result<Session, sqlx::Error> {
    Ok(sqlx::query_as!(
        Session,
        "INSERT INTO session (account, creator_ip) VALUES ($1, $2) RETURNING *",
        &account,
        &ip,
    )
    .fetch_one(conn)
    .await?)
}

/// Get a session by ID.
pub async fn get_by_id(conn: &mut PgConnection, id: &Uuid) -> Result<Option<Session>, sqlx::Error> {
    Ok(
        sqlx::query_as!(Session, "SELECT * FROM session WHERE id = $1", &id)
            .fetch_optional(conn)
            .await?,
    )
}

/// Update an existing session in the database.
/// This updates the following fields:
/// - last_seen.
/// Other fields are read-only.
pub async fn update(conn: &mut PgConnection, session: &Session) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE session SET last_seen = $1 WHERE id = $2",
        &session.last_seen,
        &session.id
    )
    .execute(conn)
    .await?;

    Ok(())
}
