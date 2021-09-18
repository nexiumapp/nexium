use nexium_lib::JsonResponder;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use sqlx::{Pool, Postgres, Transaction};
use thiserror::Error;

use crate::database::models::{Account, AuthPassword};
use crate::logic::auth::password;

/// Route to create a new account.
#[post("/new", data = "<data>")]
pub async fn route(
    data: Json<BodyData>,
    pool: &State<Pool<Postgres>>,
) -> Result<Json<Response>, RouteError> {
    let mut conn = pool.begin().await.map_err(RouteError::DatabaseError)?;

    if let Some(_) = Account::find_username(&mut conn, data.username.as_str())
        .await
        .map_err(RouteError::DatabaseError)?
    {
        return Err(RouteError::AccountExists(data.username.clone()));
    }

    let account = Account::create(&mut conn, data.username.as_str())
        .await
        .map_err(RouteError::DatabaseError)?;

    create_authentication(&mut conn, &account, &data.auth).await?;
    conn.commit().await.map_err(RouteError::DatabaseError)?;

    info!(
        "New account created for {} ({}).",
        account.username, account.id
    );

    Ok(Json(Response { account }))
}

/// Creates an authentication method with the type attached.
/// Returns Ok if the creation succeeded.
async fn create_authentication(
    conn: &mut Transaction<'_, Postgres>,
    account: &Account,
    auth: &AuthType,
) -> Result<(), RouteError> {
    match auth {
        AuthType::Password { password } => {
            if !password::validate(password.to_string()) {
                return Err(RouteError::PasswordComplexity);
            }

            let hash =
                password::hash(password.to_string()).map_err(|_| RouteError::InternalError)?;

            AuthPassword::create(conn, account.id, hash)
                .await
                .map_err(RouteError::DatabaseError)?;
        }
    }

    Ok(())
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
pub struct Response {
    account: Account,
}

/// All possible error responses for this route.
#[derive(Error, Debug, JsonResponder)]
pub enum RouteError {
    #[error("Internal server error.")]
    InternalError,
    #[error("Password is not complex enough.")]
    PasswordComplexity,
    #[error("Account with username {0} already exists.")]
    AccountExists(String),
    #[error("An internal database error occured.")]
    DatabaseError(sqlx::Error),
}

impl<'a> RouteError {
    /// Translate an route error to an code used for differentiating the errors.
    fn code(&self) -> &'a str {
        match self {
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
            RouteError::InternalError => Status::InternalServerError,
            RouteError::PasswordComplexity => Status::BadRequest,
            RouteError::AccountExists(_) => Status::BadRequest,
            RouteError::DatabaseError(_) => Status::ServiceUnavailable,
        }
    }
}
