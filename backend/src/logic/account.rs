use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use sqlx::PgConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::database;

/// Representing an account of an user.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub full_name: String,
}

impl Account {
    /// Create a new user account.
    /// This also validates the input.
    pub async fn create(conn: &mut PgConnection, username: &str) -> Result<Self, CreateError> {
        // Validate the username against the rules.
        if !Self::validate_username(username) {
            return Err(CreateError::InvalidUsername(username.to_string()));
        }

        // Check if the user already exists, and return an error if it does.
        if database::account::find_username(conn, username)
            .await?
            .is_some()
        {
            return Err(CreateError::AccountExists(username.to_string()));
        }

        // Create an new account.
        // This does not create authentication, this should be created separately.
        Ok(database::account::create(conn, username).await?)
    }

    /// Find an user by ID.
    pub async fn find(conn: &mut PgConnection, id: &Uuid) -> Result<Self, FindError> {
        let res = database::account::find(conn, id).await?;

        match res {
            Some(account) => Ok(account),
            None => Err(FindError::NotFound),
        }
    }

    /// Find an user by username.
    pub async fn find_username(conn: &mut PgConnection, username: &str) -> Result<Self, FindError> {
        let res = database::account::find_username(conn, username).await?;

        match res {
            Some(account) => Ok(account),
            None => Err(FindError::NotFound),
        }
    }

    /// Validate that a username is valid alphanumeric and of proper length.
    /// Dots in the middle of the string is allowed.
    fn validate_username(username: &str) -> bool {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("^[A-Za-z0-9]+(\\.[A-Za-z0-9]+)*$").unwrap();
        }

        if username.len() < 3 {
            return false;
        }

        if username.len() > 50 {
            return false;
        }

        if !REGEX.is_match(username) {
            return false;
        }

        true
    }
}

/// Possible errors with creating a new account.
#[derive(Error, Debug)]
pub enum CreateError {
    #[error("The username '{0}' is invalid.")]
    InvalidUsername(String),
    #[error("The account with username '{0}' already exists.")]
    AccountExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

/// Possible errors with finding an account.
#[derive(Error, Debug)]
pub enum FindError {
    #[error("The account was not found.")]
    NotFound,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}
