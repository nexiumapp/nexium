use jsonwebtoken::DecodingKey;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use thiserror::Error;

use crate::{environment::Environment, logic::session};

/// Rocket guard to make sure the request has an valid session token.
pub struct SessionTokenGuard(session::jwt::JwtToken);

#[async_trait]
impl<'r> FromRequest<'r> for SessionTokenGuard {
    type Error = SessionTokenGuardError;

    /// Provides the guard which can then be used to secure the session token routes.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the env variables from Rocket.
        let env = req.rocket().state::<Environment>().unwrap();

        // Get the authorization header, or error if it does not exist.
        let auth_header = match req.headers().get_one("Authorization") {
            Some(auth) => auth,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    SessionTokenGuardError::NoTokenProvided,
                ))
            }
        };

        // Split and check if the auth header type is "Bearer".
        let mut split_auth = auth_header.split_whitespace();
        if split_auth.next() != Some("Bearer") {
            return Outcome::Failure((Status::Unauthorized, SessionTokenGuardError::InvalidToken));
        }

        // Get the token from the second word.
        let jwt = match split_auth.next() {
            Some(token) => token.to_string(),
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    SessionTokenGuardError::InvalidToken,
                ))
            }
        };

        // Decode the session token, checking if it's valid in the meantime.
        let key = DecodingKey::from_secret(env.jwt_secret.as_bytes());
        let token = match session::jwt::JwtToken::decode(jwt, &key) {
            Ok(token) => token,
            Err(_) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    SessionTokenGuardError::InvalidToken,
                ));
            }
        };

        Outcome::Success(Self(token))
    }
}

impl From<SessionTokenGuard> for session::jwt::JwtToken {
    /// Provides easy and clean session to the wrapper session token.
    fn from(guard: SessionTokenGuard) -> session::jwt::JwtToken {
        guard.0
    }
}

/// Errors which can arrise from the session guard.
#[derive(Debug, Error)]
pub enum SessionTokenGuardError {
    #[error("No session token is provided.")]
    NoTokenProvided,
    #[error("The session token provided was invalid.")]
    InvalidToken,
}
