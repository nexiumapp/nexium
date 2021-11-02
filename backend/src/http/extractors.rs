use std::{future::Future, pin::Pin};

use actix_session::UserSession;
use actix_web::{
    dev::Payload, http::StatusCode, web, FromRequest, HttpRequest, HttpResponse, ResponseError,
};
use sqlx::{Pool, Postgres};
use thiserror::Error;
use uuid::Uuid;

use crate::logic::account::{self, Account};

use super::ApiError;

pub struct UserGuard<T>(T);

/// Userguard with the UUID generic.
/// This does not return the entire user object, but only the UUID.
/// This saves an call to the database, as long as the user object is not required in the route.
impl FromRequest for UserGuard<Uuid> {
    type Error = GuardError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();

        Box::pin(async move {
            // Get the user ID from the session.
            let user = match session.get::<Uuid>("user") {
                Ok(u) => u,
                Err(_) => return Err(GuardError::InternalError),
            };

            // Validate that the ID is set.
            let user = match user {
                Some(u) => u,
                None => return Err(GuardError::NotAuthenticated),
            };

            Ok(UserGuard(user))
        })
    }
}

impl From<UserGuard<Uuid>> for Uuid {
    fn from(val: UserGuard<Uuid>) -> Self {
        val.0
    }
}

/// Userguard with the Account generic.
/// This returns the entire user object, not only the UUID.
/// It can be useful when the route requires the user object.
impl FromRequest for UserGuard<Account> {
    type Error = GuardError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let pool = req
            .app_data::<web::Data<Pool<Postgres>>>()
            .expect("Database was not available in the guards!")
            .clone();

        Box::pin(async move {
            // Get the user ID from the session.
            let user = match session.get::<Uuid>("user") {
                Ok(u) => u,
                Err(_) => return Err(GuardError::InternalError),
            };

            // Validate that the ID is set.
            let user = match user {
                Some(u) => u,
                None => return Err(GuardError::NotAuthenticated),
            };

            // Fetch the current user object from the database.
            let mut conn = pool.acquire().await?;
            let account = Account::find(&mut conn, &user).await?;

            Ok(UserGuard(account))
        })
    }
}

impl From<UserGuard<Account>> for Account {
    fn from(val: UserGuard<Account>) -> Self {
        val.0
    }
}

/// All possible error responses for this route.
#[derive(Error, Debug)]
pub enum GuardError {
    #[error("You are not logged in.")]
    NotAuthenticated,
    #[error("Internal server error.")]
    InternalError,
    #[error("Internal server error.")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> ApiError<'a> for GuardError {
    /// Convert the enum variant to a machine-readable name for the client.
    fn error_code(&self) -> &'a str {
        match self {
            GuardError::NotAuthenticated => "notauthenticated",
            GuardError::InternalError => "internalerror",
            GuardError::DatabaseError(_) => "databaseerror",
        }
    }
}

impl ResponseError for GuardError {
    /// Translate a route error to a HTTP status.
    fn status_code(&self) -> StatusCode {
        match self {
            GuardError::NotAuthenticated => StatusCode::UNAUTHORIZED,
            GuardError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GuardError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Generate the error response.
    fn error_response(&self) -> HttpResponse {
        self.json()
    }
}

impl From<account::FindError> for GuardError {
    /// Map find errors to route errors.
    fn from(e: account::FindError) -> Self {
        match e {
            account::FindError::NotFound => GuardError::NotAuthenticated,
            account::FindError::DatabaseError(e) => GuardError::DatabaseError(e),
        }
    }
}
