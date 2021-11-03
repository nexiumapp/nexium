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

/// Create a new account.
#[post("/new")]
async fn new_account(
    data: Json<BodyData>,
    user: Option<UserGuard<Uuid>>,
    session: Session,
    pool: web::Data<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    // Check that the current session is not logged in.
    if user.is_some() {
        return Err(RouteError::LoggedIn);
    }

    // Start an database transaction.
    let mut conn = pool.begin().await.map_err(RouteError::DatabaseError)?;

    // Create the account.
    // Note that there is no authentication data is added, this will be done in the next step.
    // Because of the transaction we can be sure there won't be an account created without authentication.
    let account = account::Account::create(&mut conn, &data.username).await?;

    // Create the authentication.
    // The specific method depends on the type.
    match &data.auth {
        AuthType::Password { password } => {
            auth::password::AuthPassword::create(&mut conn, &account, password).await?
        }
    };

    // Commit the changes to the database.
    conn.commit().await.map_err(RouteError::DatabaseError)?;

    // Attach the user ID to the current session.
    session
        .insert("user", account.id)
        .map_err(|_| RouteError::InternalError)?;

    info!(
        "New account created for {} ({}).",
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
    #[error("The given username is not valid.")]
    InvalidUsername,
    #[error("Password is not complex enough.")]
    PasswordComplexity,
    #[error("Internal server error.")]
    InternalError,
    #[error("Account with username {0} already exists.")]
    AccountExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> ApiError<'a> for RouteError {
    /// Convert the enum variant to a machine-readable name for the client.
    fn error_code(&self) -> &'a str {
        match self {
            RouteError::LoggedIn => "loggedin",
            RouteError::InvalidUsername => "invalidusername",
            RouteError::InternalError => "internalerror",
            RouteError::PasswordComplexity => "passwordcomplexity",
            RouteError::AccountExists(_) => "accountexists",
            RouteError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl ResponseError for RouteError {
    /// Translate a route error to a HTTP status.
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::LoggedIn => StatusCode::BAD_REQUEST,
            RouteError::InvalidUsername => StatusCode::BAD_REQUEST,
            RouteError::PasswordComplexity => StatusCode::BAD_REQUEST,
            RouteError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::AccountExists(_) => StatusCode::BAD_REQUEST,
            RouteError::DatabaseError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Generate the error response.
    fn error_response(&self) -> HttpResponse {
        self.json()
    }
}

/// Convert the internal error to an route error.
impl From<account::CreateError> for RouteError {
    fn from(err: account::CreateError) -> Self {
        match err {
            account::CreateError::InvalidUsername(_) => RouteError::InvalidUsername,
            account::CreateError::AccountExists(username) => RouteError::AccountExists(username),
            account::CreateError::DatabaseError(e) => RouteError::DatabaseError(e),
        }
    }
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
