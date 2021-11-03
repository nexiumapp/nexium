use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};
use uuid::Uuid;

use crate::http::UserGuard;

/// Log out the current user.
#[post("/logout")]
async fn logout(_uuid: UserGuard<Uuid>, session: Session) -> impl Responder {
    // Purge the session.
    session.purge();

    HttpResponse::Ok()
}
