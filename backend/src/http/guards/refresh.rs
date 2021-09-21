use jsonwebtoken::DecodingKey;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::{environment::Environment, logic::session};

/// Refresh token guard to be used with a route which requires a valid refresh token.
pub struct RefreshTokenGuard(session::refresh::RefreshToken);

#[async_trait]
impl<'r> FromRequest<'r> for RefreshTokenGuard {
    type Error = RefreshTokenGuardError;

    /// Provides the guard which can then be used to secure the refresh token routes.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the database and the env variables from Rocket.
        let pool = req.rocket().state::<Pool<Postgres>>().unwrap();
        let env = req.rocket().state::<Environment>().unwrap();

        // Get the authorization header, or error if it does not exist.
        let auth_header = match req.headers().get_one("Authorization") {
            Some(auth) => auth,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    RefreshTokenGuardError::NoTokenProvided,
                ))
            }
        };

        // Split and check if the auth header type is "Bearer".
        let mut split_auth = auth_header.split_whitespace();
        if split_auth.next() != Some("Bearer") {
            return Outcome::Failure((Status::Unauthorized, RefreshTokenGuardError::InvalidToken));
        }

        // Get the token from the second word.
        let jwt = match split_auth.next() {
            Some(token) => token.to_string(),
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    RefreshTokenGuardError::InvalidToken,
                ))
            }
        };

        // Aquire an database connection.
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(e) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    RefreshTokenGuardError::DatabaseError(e),
                ))
            }
        };

        // Decode the refresh token, checking if it's valid in the meantime.
        let key = DecodingKey::from_secret(env.jwt_secret.as_bytes());
        let token = match session::refresh::RefreshToken::decode(&mut conn, jwt, &key).await {
            Ok(token) => token,
            Err(_) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    RefreshTokenGuardError::InvalidToken,
                ));
            }
        };

        Outcome::Success(RefreshTokenGuard(token))
    }
}

impl From<RefreshTokenGuard> for session::refresh::RefreshToken {
    /// Provides easy and clean access to the wrapper refresh token.
    fn from(guard: RefreshTokenGuard) -> session::refresh::RefreshToken {
        guard.0
    }
}

/// Errors which can arrise from the refresh token guard.
#[derive(Error, Debug)]
pub enum RefreshTokenGuardError {
    #[error("No refresh token was provided.")]
    NoTokenProvided,
    #[error("The provided token was invalid.")]
    InvalidToken,
    #[error("An internal server error occured.")]
    InternalError,
    #[error("An database error occured.")]
    DatabaseError(sqlx::Error),
}
