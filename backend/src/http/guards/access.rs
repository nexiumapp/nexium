use jsonwebtoken::DecodingKey;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use thiserror::Error;

use crate::{environment::Environment, logic::session};

/// Rocket guard to make sure the request has an valid access token.
pub struct AccessTokenGuard(session::access::AccessToken);

#[async_trait]
impl<'r> FromRequest<'r> for AccessTokenGuard {
    type Error = AccessTokenGuardError;

    /// Provides the guard which can then be used to secure the access token routes.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the env variables from Rocket.
        let env = req.rocket().state::<Environment>().unwrap();

        // Get the authorization header, or error if it does not exist.
        let auth_header = match req.headers().get_one("Authorization") {
            Some(auth) => auth,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    AccessTokenGuardError::NoTokenProvided,
                ))
            }
        };

        // Split and check if the auth header type is "Bearer".
        let mut split_auth = auth_header.split_whitespace();
        if split_auth.next() != Some("Bearer") {
            return Outcome::Failure((Status::Unauthorized, AccessTokenGuardError::InvalidToken));
        }

        // Get the token from the second word.
        let jwt = match split_auth.next() {
            Some(token) => token.to_string(),
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    AccessTokenGuardError::InvalidToken,
                ))
            }
        };

        // Decode the refresh token, checking if it's valid in the meantime.
        let key = DecodingKey::from_secret(env.jwt_secret.as_bytes());
        let token = match session::access::AccessToken::decode(jwt, &key) {
            Ok(token) => token,
            Err(_) => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    AccessTokenGuardError::InvalidToken,
                ));
            }
        };

        Outcome::Success(AccessTokenGuard(token))
    }
}

impl From<AccessTokenGuard> for session::access::AccessToken {
    /// Provides easy and clean access to the wrapper access token.
    fn from(guard: AccessTokenGuard) -> session::access::AccessToken {
        guard.0
    }
}

/// Errors which can arrise from the access guard.
#[derive(Debug, Error)]
pub enum AccessTokenGuardError {
    #[error("No access token is provided.")]
    NoTokenProvided,
    #[error("The access token provided was invalid.")]
    InvalidToken,
}
