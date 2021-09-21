use crate::{
    http::guards::{AccessTokenGuard, AccessTokenGuardError},
    logic,
};
use nexium_lib::JsonResponder;
use rocket::{http::Status, serde::json::Json, State};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use thiserror::Error;

/// Route to create a new account.
#[get("/whoami")]
pub async fn route(
    access: Result<AccessTokenGuard, AccessTokenGuardError>,
    pool: &State<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    // Error out if the access token was invalid.
    let access_token: logic::session::access::AccessToken = access?.into();

    let mut conn = pool.acquire().await?;
    let account = logic::account::Account::find(&mut conn, &access_token.claims.account).await?;

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
    DatabaseError(sqlx::Error),
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

impl From<AccessTokenGuardError> for RouteError {
    /// Convert an access token error to an access denied.
    fn from(_: AccessTokenGuardError) -> Self {
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

impl From<sqlx::Error> for RouteError {
    /// Wrap the sqlx errors in the correct route error.
    fn from(e: sqlx::Error) -> Self {
        RouteError::DatabaseError(e)
    }
}
