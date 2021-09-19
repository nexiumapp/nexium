use argon2::{
    password_hash::{errors::Error, rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use sqlx::PgConnection;
use thiserror::Error;
use zxcvbn::zxcvbn;

use crate::database::models::{Account, AuthPassword};

pub async fn create(
    conn: &mut PgConnection,
    account: &Account,
    password: &String,
) -> Result<AuthPassword, CreateError> {
    // First validate the password complexity.
    if !validate(password.to_string()) {
        return Err(CreateError::PasswordComplexity);
    }

    // Hash the password for storage.
    let hash = hash(password.to_string()).map_err(|_| CreateError::HashError)?;

    // Save the password to the database.
    let auth = AuthPassword::create(conn, account.id, hash)
        .await
        .map_err(CreateError::DatabaseError)?;

    Ok(auth)
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("The password is not complex enough")]
    PasswordComplexity,
    #[error("The password could not be hashed")]
    HashError,
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}

/// Validate the complexity of the password.
/// This uses the Zxcvbn library to check, and limits the length to 200 characters.
pub fn validate(plaintext: String) -> bool {
    if plaintext.len() > 200 {
        return false;
    }

    let complexity = zxcvbn(plaintext.as_str(), &[]);
    if complexity.is_err() {
        return false;
    }

    complexity.unwrap().score() > 3
}

/// Compare an hashed password with an plaintext.
/// Only returns Ok when the password is valid, error otherwise.
pub fn _compare(hash: String, plaintext: String) -> Result<(), Error> {
    let parsed = PasswordHash::new(&hash)?;
    let context = create_context();

    context.verify_password(plaintext.as_bytes(), &parsed)
}

/// Hash an plaintext to an format suitable for storage.
pub fn hash(plaintext: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let context = create_context();

    Ok(context
        .hash_password(plaintext.as_bytes(), &salt)?
        .to_string())
}

fn create_context<'a>() -> Argon2<'a> {
    Argon2::new(
        Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    )
}
