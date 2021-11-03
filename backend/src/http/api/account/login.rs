use actix_session::Session;
use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

use crate::http::{ApiError, UserGuard};
use crate::logic::{account, auth};

/// Log into an existing account.
#[post("/login")]
async fn login(
    data: Json<BodyData>,
    user: Option<UserGuard<Uuid>>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    // Check that the current session is not logged in.
    if user.is_some() {
        return Err(RouteError::LoggedIn);
    }

    // Get an database connection.
    // A transation is not needed here, as it's read only.
    let mut conn = pool.acquire().await.map_err(RouteError::DatabaseError)?;

    // Find the account associated with the user.
    let account = account::Account::find_username(&mut conn, &data.username).await?;

    // Run the authentication for this type.
    match &data.auth {
        AuthType::Password { password } => {
            auth::password::AuthPassword::authenticate(&mut conn, &account, password).await?;
        }
    };

    // Attach the user ID to the current session.
    session
        .insert("user", account.id)
        .map_err(|_| RouteError::InternalError)?;

    info!(
        "New successful login for {} ({}).",
        account.username, account.id
    );

    Ok(Json(Response { account }))
}

/// Requested data for this route.
#[derive(Deserialize)]
struct BodyData {
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
struct Response {
    account: account::Account,
}

/// All possible error responses for this route.
#[derive(Error, Debug)]
enum RouteError {
    #[error("Session is already logged in.")]
    LoggedIn,
    #[error("Invalid credentials provided.")]
    LoginFailed,
    #[error("Internal server error.")]
    InternalError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> ApiError<'a> for RouteError {
    /// Convert the enum variant to a machine-readable name for the client.
    fn error_code(&self) -> &'a str {
        match self {
            RouteError::LoggedIn => "loggedin",
            RouteError::LoginFailed => "loginfailed",
            RouteError::InternalError => "internalerror",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl ResponseError for RouteError {
    /// Translate a route error to a HTTP status.
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::LoggedIn => StatusCode::BAD_REQUEST,
            RouteError::LoginFailed => StatusCode::BAD_REQUEST,
            RouteError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Generate the error response.
    fn error_response(&self) -> HttpResponse {
        self.json()
    }
}

/// Convert the internal error to an route error.
impl From<account::FindError> for RouteError {
    fn from(err: account::FindError) -> Self {
        match err {
            account::FindError::NotFound => RouteError::LoginFailed,
            account::FindError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}

/// Convert the internal error to an route error.
impl From<auth::password::AuthenticateError> for RouteError {
    fn from(err: auth::password::AuthenticateError) -> Self {
        match err {
            auth::password::AuthenticateError::NoPasswordAuth => RouteError::LoginFailed,
            auth::password::AuthenticateError::IncorrectPassword(_) => RouteError::LoginFailed,
            auth::password::AuthenticateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
}
