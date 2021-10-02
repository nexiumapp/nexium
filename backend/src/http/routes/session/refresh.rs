use jsonresponder::JsonResponder;
use jsonwebtoken::EncodingKey;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use thiserror::Error;

use crate::environment::Environment;
use crate::http::guards::{RefreshTokenGuard, RefreshTokenGuardError};
use crate::logic::session;
use crate::logic::session::refresh::RefreshToken;

/// Route to create a new account.
#[post("/refresh")]
pub async fn route(
    refresh_token: Result<RefreshTokenGuard, RefreshTokenGuardError>,
    env: &State<Environment>,
) -> Result<Json<Response>, RouteError> {
    // Unwrap the refresh token to error out when the token is invalid.
    let refresh_token: RefreshToken = refresh_token?.into();

    // Encode the refresh token into a new access token.
    let key = EncodingKey::from_secret(env.jwt_secret.as_bytes());
    let access = session::access::AccessToken::encode(&refresh_token, &key)?;

    debug!(
        "Session {} ({}) refreshed.",
        access.claims.session, access.claims.account
    );

    Ok(Json(Response {
        access_token: access.jwt,
    }))
}

/// Success response of this route.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    access_token: String,
}

/// All possible error responses for this route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("Access denied.")]
    AccessDenied,
    #[error("An internal error occured.")]
    InternalError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
            RouteError::AccessDenied => "accessdenied",
            RouteError::InternalError => "internalerror",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    /// Translate an route error to an Rocket status.
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::AccessDenied => Status::Unauthorized,
            RouteError::InternalError => Status::InternalServerError,
            RouteError::DatabaseError(_) => Status::ServiceUnavailable,
        }
    }
}

impl From<RefreshTokenGuardError> for RouteError {
    // Translate an refresh token error to an route error.
    fn from(err: RefreshTokenGuardError) -> Self {
        match err {
            RefreshTokenGuardError::NoTokenProvided => RouteError::AccessDenied,
            RefreshTokenGuardError::InvalidToken => RouteError::AccessDenied,
            RefreshTokenGuardError::InternalError => RouteError::InternalError,
            RefreshTokenGuardError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for RouteError {
    // Translate an jwt error into an route error.
    // As JWT errors should never happen, this is an internal error.
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        RouteError::InternalError
    }
}
