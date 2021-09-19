use std::net::SocketAddr;

use nexium_lib::JsonResponder;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::database::models::Account;
use crate::logic::{account, auth, session};

/// Route to create a new account.
#[post("/new", data = "<data>")]
pub async fn route(
    data: Json<BodyData>,
    pool: &State<Pool<Postgres>>,
    addr: SocketAddr,
) -> Result<Json<Response>, RouteError> {
    // Start an database transaction.
    let mut conn = pool.begin().await.map_err(RouteError::DatabaseError)?;

    // Create the account.
    // Note that there is no authentication data is added, this will be done in the next step.
    // Because of the transaction we can be sure there won't be an account created without authentication.
    let account = account::create(&mut conn, &data.username).await?;

    // Create the authentication.
    // The specific method depends on the type.
    match &data.auth {
        AuthType::Password { password } => {
            auth::password::create(&mut conn, &account, &password).await?
        }
    };

    // Now create the session associated with the account.
    let session = session::create(&mut conn, &account, &addr).await?;

    // Commit the changes to the database.
    conn.commit().await.map_err(RouteError::DatabaseError)?;

    info!(
        "New account created for {} ({}).",
        account.username, account.id
    );

    Ok(Json(Response {
        refresh_token: session.secret,
        account,
    }))
}

/// Requested data for this route.
#[derive(Deserialize)]
pub struct BodyData {
    username: String,
    auth: AuthType,
}

/// Represents the different methods of authentication.
/// One is required to create an account.
#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum AuthType {
    Password { password: String },
}

/// Success response of this route.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    account: Account,
    refresh_token: String,
}

/// All possible error responses for this route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("The given username is not valid.")]
    InvalidUsername,
    #[error("Internal server error.")]
    InternalError,
    #[error("Password is not complex enough.")]
    PasswordComplexity,
    #[error("Account with username {0} already exists.")]
    AccountExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}

/// Convert the internal error to an route error.
impl From<auth::password::CreateError> for RouteError {
    fn from(err: auth::password::CreateError) -> Self {
        match err {
            auth::password::CreateError::PasswordComplexity => RouteError::PasswordComplexity,
            auth::password::CreateError::HashError => RouteError::InternalError,
            auth::password::CreateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

/// Convert the internal error to an route error.
impl From<session::CreateError> for RouteError {
    fn from(err: session::CreateError) -> Self {
        match err {
            session::CreateError::IpParseError => RouteError::InternalError,
            session::CreateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

/// Convert the internal error to an route error.
impl From<account::CreateError> for RouteError {
    fn from(err: account::CreateError) -> Self {
        match err {
            account::CreateError::InvalidUsername(_) => RouteError::InvalidUsername,
            account::CreateError::UserExists(username) => RouteError::AccountExists(username),
            account::CreateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
            RouteError::InvalidUsername => "invalidusername",
            RouteError::InternalError => "internalerror",
            RouteError::PasswordComplexity => "passwordcomplexity",
            RouteError::AccountExists(_) => "accountexists",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    /// Translate an route error to an Rocket status.
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::InvalidUsername => Status::BadRequest,
            RouteError::InternalError => Status::InternalServerError,
            RouteError::PasswordComplexity => Status::BadRequest,
            RouteError::AccountExists(_) => Status::BadRequest,
            RouteError::DatabaseError(_) => Status::ServiceUnavailable,
        }
    }
}
