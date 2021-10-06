use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::Serialize;
use sqlx::PgConnection;
use thiserror::Error;
use uuid::Uuid;
use zxcvbn::zxcvbn;

use crate::{database, logic::account::Account};

/// Represents an optional password authentication method for an user account.
#[derive(Debug, Serialize)]
pub struct AuthPassword {
    pub account: Uuid,
    pub hash: String,
}

impl AuthPassword {
    /// Create a new password authentication method associated with an account.
    pub async fn create(
        conn: &mut PgConnection,
        account: &Account,
        password: &str,
    ) -> Result<Self, CreateError> {
        // First validate the password complexity.
        if !Self::validate(password.to_string(), &[account.username.as_str()]) {
            return Err(CreateError::PasswordComplexity);
        }

        // Hash the password for storage.
        let hash = Self::hash(password.to_string()).map_err(|_| CreateError::HashError)?;

        // Save the password to the database.
        let auth = database::auth_password::create(conn, account.id, hash).await?;

        Ok(auth)
    }

    /// Authenticate an account with the password.
    /// This fetches and compares the hash associated with this account.
    pub async fn authenticate(
        conn: &mut PgConnection,
        account: &Account,
        plaintext: &str,
    ) -> Result<(), AuthenticateError> {
        let pw = database::auth_password::get(conn, account.id)
            .await?
            .ok_or(AuthenticateError::NoPassword)?;
        AuthPassword::compare(pw.hash, plaintext)?;

        Ok(())
    }

    /// Compare an hashed password with an plaintext.
    /// Only returns Ok when the password is valid, error otherwise.
    fn compare(hash: String, plaintext: &str) -> Result<(), password_hash::Error> {
        let parsed = PasswordHash::new(&hash)?;
        let context = Self::create_context();

        context.verify_password(plaintext.as_bytes(), &parsed)
    }

    /// Validate the complexity of the password.
    /// This uses the Zxcvbn library to check, and limits the length to 200 characters.
    fn validate(plaintext: String, user_inputs: &[&str]) -> bool {
        if plaintext.len() > 200 {
            return false;
        }

        let complexity = zxcvbn(plaintext.as_str(), user_inputs);
        if complexity.is_err() {
            return false;
        }

        complexity.unwrap().score() >= 3
    }

    /// Hash an plaintext to an format suitable for storage.
    fn hash(plaintext: String) -> Result<String, password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let context = Self::create_context();

        Ok(context
            .hash_password(plaintext.as_bytes(), &salt)?
            .to_string())
    }

    /// Create a new Argon2 context used for hashing.
    fn create_context<'a>() -> Argon2<'a> {
        Argon2::new(
            Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::default(),
        )
    }
}

/// Possible errors which occur when creating an password authentication method.
#[derive(Error, Debug)]
pub enum CreateError {
    #[error("The password is not complex enough")]
    PasswordComplexity,
    #[error("The password could not be hashed")]
    HashError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

/// Possible errors which occur when logging in.
#[derive(Error, Debug)]
pub enum AuthenticateError {
    #[error("This account does not have password authentication.")]
    NoPassword,
    #[error("The given password is incorrect.")]
    IncorrectPassword(#[from] password_hash::Error),
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}
