use crate::{
    http::guards::{SessionTokenGuard, SessionTokenGuardError},
    logic,
};
use jsonresponder::JsonResponder;
use rocket::{http::Status, serde::json::Json, State};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use thiserror::Error;

/// Route to create a new account.
#[get("/whoami")]
pub async fn route(
    session: Result<SessionTokenGuard, SessionTokenGuardError>,
    pool: &State<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    // Error out if the session token was invalid.
    let token: logic::session::jwt::JwtToken = session?.into();

    // Acquire an connection, not an transaction as it's just read only.
    let mut conn = pool.acquire().await?;
    let account = logic::account::Account::find(&mut conn, &token.claims.account).await?;

    Ok(Json(Response { account }))
}

/// Succes response of this route.
#[derive(Serialize)]
pub struct Response {
    account: logic::account::Account,
}

/// Possible errors on the route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("Access denied.")]
    AccessDenied,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
            RouteError::AccessDenied => "accessdenied",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    /// Translate an route error to an Rocket status.
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::AccessDenied => Status::Unauthorized,
            RouteError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}

impl From<SessionTokenGuardError> for RouteError {
    /// Convert an session token error to an access denied.
    fn from(_: SessionTokenGuardError) -> Self {
        RouteError::AccessDenied
    }
}

impl From<logic::account::FindError> for RouteError {
    /// Map find errors to route errors.
    fn from(e: logic::account::FindError) -> Self {
        match e {
            logic::account::FindError::NotFound => RouteError::AccessDenied,
            logic::account::FindError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}
