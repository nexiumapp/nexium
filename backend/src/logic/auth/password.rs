use argon2::{
    password_hash::{errors::Error, rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
};
use zxcvbn::zxcvbn;

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
