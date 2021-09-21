use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

mod tables;

pub use tables::*;

static MIGRATOR: Migrator = sqlx::migrate!();

/// Establish an connection pool with the database.
pub async fn connect(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(15)
        .connect_timeout(Duration::new(5, 0))
        .connect(url)
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
