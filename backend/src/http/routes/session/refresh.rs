use jsonresponder::JsonResponder;
use jsonwebtoken::EncodingKey;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::environment::Environment;
use crate::http::guards::{UncheckedSessionGuard, UncheckedSessionGuardError};
use crate::logic::session::jwt::{JwtToken, RenewError};

/// Route to create a new account.
#[post("/refresh")]
pub async fn route(
    session: Result<UncheckedSessionGuard, UncheckedSessionGuardError>,
    pool: &State<Pool<Postgres>>,
    env: &State<Environment>,
) -> Result<Json<Response>, RouteError> {
    // Unwrap the session to error out when the token is invalid.
    let session: JwtToken = session?.into();

    // Renew the session, generate a new JWT.
    let key = EncodingKey::from_secret(env.jwt_secret.as_bytes());
    let mut conn = pool.acquire().await?;
    let renewed = session.renew(&mut conn, &key).await?;

    debug!(
        "Session {} ({}) renewed.",
        renewed.claims.session, renewed.claims.account
    );

    Ok(Json(Response { token: renewed.jwt }))
}

/// Success response of this route.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    token: String,
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

impl From<UncheckedSessionGuardError> for RouteError {
    // Translate an session token error to an route error.
    fn from(err: UncheckedSessionGuardError) -> Self {
        match err {
            UncheckedSessionGuardError::InvalidToken => RouteError::AccessDenied,
            UncheckedSessionGuardError::NoTokenProvided => RouteError::AccessDenied,
        }
    }
}

impl From<RenewError> for RouteError {
    // Translate a renew error to an route error.
    fn from(err: RenewError) -> Self {
        match err {
            RenewError::SessionUnknown => RouteError::AccessDenied,
            RenewError::TokenError(_) => RouteError::InternalError,
            RenewError::DatabaseError(e) => RouteError::DatabaseError(e),
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
