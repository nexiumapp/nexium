use jsonwebtoken::DecodingKey;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use thiserror::Error;

use crate::{environment::Environment, logic::session};

/// Rocket guard to make sure the request has an valid session token.
pub struct UncheckedSessionGuard(session::jwt::JwtToken);

#[async_trait]
impl<'r> FromRequest<'r> for UncheckedSessionGuard {
    type Error = UncheckedSessionGuardError;

    /// Provides the guard which can then be used to secure the session token routes.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the env variable from Rocket.
        let env = req.rocket().state::<Environment>().unwrap();

        // Get the authorization header, or error if it does not exist.
        let auth_header = match req.headers().get_one("Authorization") {
            Some(auth) => auth,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    UncheckedSessionGuardError::NoTokenProvided,
                ))
            }
        };

        // Split and check if the auth header type is "Bearer".
        let mut split_auth = auth_header.split_whitespace();
        if split_auth.next() != Some("Bearer") {
            return Outcome::Failure((
                Status::Unauthorized,
                UncheckedSessionGuardError::InvalidToken,
            ));
        }

        // Get the token from the second word.
        let jwt = match split_auth.next() {
            Some(token) => token.to_string(),
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    UncheckedSessionGuardError::InvalidToken,
                ))
            }
        };

        // Decode the session token unchecked.
        let key = DecodingKey::from_secret(env.jwt_secret.as_bytes());
        let token = match session::jwt::JwtToken::decode_unchecked(jwt, &key) {
            Ok(token) => token,
            Err(_) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    UncheckedSessionGuardError::InvalidToken,
                ));
            }
        };

        Outcome::Success(Self(token))
    }
}

impl From<UncheckedSessionGuard> for session::jwt::JwtToken {
    /// Provides easy and clean session to the wrapper session token.
    fn from(guard: UncheckedSessionGuard) -> session::jwt::JwtToken {
        guard.0
    }
}

/// Errors which can arrise from the session guard.
#[derive(Debug, Error)]
pub enum UncheckedSessionGuardError {
    #[error("No session token is provided.")]
    NoTokenProvided,
    #[error("The session token provided was invalid.")]
    InvalidToken,
}
