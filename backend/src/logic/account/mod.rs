use sqlx::PgConnection;
use thiserror::Error;

use crate::database::models::Account;

/// Create a new user account.
/// This also validates the input.
pub async fn create(conn: &mut PgConnection, username: &str) -> Result<Account, CreateError> {
    // Validate the username against the rules.
    if !validate_username(username) {
        return Err(CreateError::InvalidUsername(username.to_string()));
    }

    // Check if the user already exists, and return an error if it does.
    if Account::find_username(conn, username)
        .await
        .map_err(CreateError::DatabaseError)?
        .is_some()
    {
        return Err(CreateError::UserExists(username.to_string()));
    }

    // Create an new account.
    // This does not create authentication, this should be created separately.
    let account = Account::create(conn, username)
        .await
        .map_err(CreateError::DatabaseError)?;

    Ok(account)
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("The username '{0}' is invalid.")]
    InvalidUsername(String),
    #[error("The user '{0}' already exists.")]
    UserExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}

/// Validate that a username is valid alphanumeric and of proper length.
pub fn validate_username(username: &str) -> bool {
    if username.len() < 3 {
        return false;
    }

    if username.len() > 50 {
        return false;
    }

    if !username.chars().all(char::is_alphanumeric) {
        return false;
    }

    true
}
