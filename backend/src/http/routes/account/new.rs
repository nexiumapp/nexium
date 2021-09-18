use nexium_lib::JsonResponder;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::database::models::Account;

/// Route to create a new account.
#[post("/new", data = "<data>")]
pub async fn route(
    data: Json<BodyData>,
    pool: &State<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    let mut conn = pool.acquire().await.map_err(RouteError::DatabaseError)?;

    match Account::find_username(&mut conn, data.username.as_str())
        .await
        .map_err(RouteError::DatabaseError)?
    {
        Some(_) => (),
        None => return Err(RouteError::AccountExists(data.username.clone())),
    }

    let account = Account::create(&mut conn, data.username.as_str())
        .await
        .map_err(RouteError::DatabaseError)?;

    info!(
        "New account created for {} ({}).",
        account.username, account.id
    );

    Ok(Json(Response { account }))
}

/// Requested data for this route.
#[derive(Deserialize)]
pub struct BodyData {
    username: String,
}

/// Success response of this route.
#[derive(Serialize)]
pub struct Response {
    account: Account,
}

/// All possible error responses for this route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("Account with username {0} already exists.")]
    AccountExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
            RouteError::AccountExists(_) => "accountexists",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    /// Translate an route error to an Rocket status.
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::AccountExists(_) => Status::BadRequest,
            RouteError::DatabaseError(_) => Status::ServiceUnavailable,
        }
    }
}
