use argon2::{
    password_hash::{errors::Error, rand_core::OsRng, SaltString},
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
        let auth = database::auth_password::create(conn, account.id, hash)
            .await
            .map_err(CreateError::DatabaseError)?;

        Ok(auth)
    }

    /// Compare an hashed password with an plaintext.
    /// Only returns Ok when the password is valid, error otherwise.
    pub fn _compare(hash: String, plaintext: String) -> Result<(), Error> {
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
    fn hash(plaintext: String) -> Result<String, Error> {
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
