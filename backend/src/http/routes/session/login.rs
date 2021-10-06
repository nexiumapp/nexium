use std::net::SocketAddr;

use jsonresponder::JsonResponder;
use jsonwebtoken::EncodingKey;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Pool, Postgres};
use thiserror::Error;

use crate::environment::Environment;
use crate::logic::{account, auth, session};

/// Route to create a new session by logging in.
#[post("/login", data = "<data>")]
pub async fn route(
    addr: SocketAddr,
    data: Json<BodyData>,
    pool: &State<Pool<Postgres>>,
    env: &State<Environment>,
) -> Result<Json<Response>, RouteError> {
    // Start an database transaction.
    let mut conn = pool.begin().await.map_err(RouteError::DatabaseError)?;

    // Find the account associated with the user.
    let account = account::Account::find_username(&mut conn, &data.username).await?;

    // Run the authentication for this type.
    match &data.auth {
        AuthType::Password { password } => {
            auth::password::AuthPassword::authenticate(&mut conn, &account, password).await?
        }
    };

    // Now create the session associated with the account.
    let session = session::Session::create(&mut conn, &account, &addr).await?;
    let key = &EncodingKey::from_secret(env.jwt_secret.as_bytes());
    let refresh = session::refresh::RefreshToken::encode(session, key)?;

    // Commit the changes to the database.
    conn.commit().await.map_err(RouteError::DatabaseError)?;

    // Create a new access token with the new refresh token.
    let access = session::access::AccessToken::encode(&refresh, key)?;

    info!(
        "New successful login for {} ({}).",
        account.username, account.id
    );

    Ok(Json(Response {
        refresh_token: refresh.jwt,
        access_token: access.jwt,
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
/// One is required to login to an account.
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
    account: account::Account,
    refresh_token: String,
    access_token: String,
}

/// All possible error responses for this route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("The provided user was unknown.")]
    UnknownUser,
    #[error("The provided password was invalid.")]
    PasswordError,
    #[error("An internal error occured.")]
    InternalError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
            RouteError::UnknownUser => "unknownuser",
            RouteError::PasswordError => "passworderror",
            RouteError::InternalError => "internalerror",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl From<RouteError> for Status {
    /// Translate an route error to an Rocket status.
    fn from(err: RouteError) -> Self {
        match err {
            RouteError::UnknownUser => Status::BadGateway,
            RouteError::PasswordError => Status::BadGateway,
            RouteError::InternalError => Status::InternalServerError,
            RouteError::DatabaseError(_) => Status::ServiceUnavailable,
        }
    }
}

/// Convert the internal error to an route error.
impl From<auth::password::AuthenticateError> for RouteError {
    fn from(err: auth::password::AuthenticateError) -> Self {
        match err {
            auth::password::AuthenticateError::NoPassword => RouteError::PasswordError,
            auth::password::AuthenticateError::IncorrectPassword(_) => RouteError::PasswordError,
            auth::password::AuthenticateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

/// Convert the internal error to an route error.
impl From<account::FindError> for RouteError {
    fn from(err: account::FindError) -> Self {
        match err {
            account::FindError::NotFound => RouteError::UnknownUser,
            account::FindError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

// Translate an jwt error into an route error.
// As JWT errors should never happen, this is an internal error.
impl From<jsonwebtoken::errors::Error> for RouteError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        RouteError::InternalError
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
